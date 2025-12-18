//! # Email Service
//!
//! Async email sending for notifications and transactional emails.
//!
//! ## Features
//!
//! - Non-blocking email delivery via Lettre SMTP
//! - Automatic retry with exponential backoff
//! - Graceful error handling with detailed logging
//! - Support for plain text and HTML emails
//! - Global singleton pattern for efficient resource usage
//!
//! ## Supported Templates
//!
//! - Contact form notifications
//! - Order confirmations (planned)
//! - Payment receipts (planned)
//! - Newsletter dispatch (planned)
//!
//! ## Configuration
//!
//! The service is configured via environment variables:
//! - `SMTP_HOST` - SMTP server hostname (default: smtp.gmail.com)
//! - `SMTP_PORT` - SMTP port (default: 587)
//! - `SMTP_USERNAME` - SMTP authentication username (required)
//! - `SMTP_PASSWORD` - SMTP authentication password (required)
//! - `FROM_EMAIL` - Sender email address (default: noreply@xftradesmen.com)
//! - `CONTACT_NOTIFICATION_EMAIL` - Recipient for contact form submissions
//!
//! ## Usage
//!
//! ```rust
//! use lib_core::email::email_service;
//!
//! // Get global instance (lazy initialized)
//! let service = email_service();
//!
//! // Send contact notification (async, non-blocking)
//! tokio::spawn(async move {
//!     if let Ok(svc) = service {
//!         let _ = svc.send_contact_notification(
//!             "John Doe",
//!             "john@example.com",
//!             Some("Question"),
//!             "Hello, I have a question"
//!         ).await;
//!     }
//! });
//! ```
//!
//! ## Retry Strategy
//!
//! Failures are automatically retried up to 3 times with exponential backoff:
//! - Attempt 1: Immediate
//! - Attempt 2: Wait 100ms
//! - Attempt 3: Wait 200ms
//!
//! All attempts are logged for debugging purposes.
//!
//! ## Error Handling
//!
//! Email failures are typically spawned in background tasks and logged
//! but do not cause request failures. This ensures contact form submissions
//! or other operations succeed even if email delivery is temporarily unavailable.

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{error, info, warn};

/// Email service for sending transactional emails.
///
/// This service is expensive to create (SMTP connection setup),
/// so it should be initialized once and reused. Use the
/// [`email_service()`] function to get the global singleton instance.
///
/// # Example
///
/// ```rust
/// use lib_core::email::email_service;
///
/// let service = email_service()?;
/// service.send_contact_notification(
///     "John", "john@example.com", Some("Hello"), "Message"
/// ).await?;
/// ```
#[derive(Debug, Clone)]
pub struct EmailService {
    /// SMTP transport for sending emails
    mailer: SmtpTransport,
    /// Default sender email address
    from_email: String,
}

/// Email message to be sent.
///
/// This struct represents a complete email message ready for delivery.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailMessage {
    /// Recipient email address
    pub to: String,
    /// Email subject line
    pub subject: String,
    /// Email body content
    pub body: String,
    /// Content type (e.g., "text/plain; charset=utf8")
    pub content_type: String,
}

impl EmailService {
    /// Create a new email service from environment variables.
    ///
    /// # Environment Variables
    ///
    /// - `SMTP_HOST` - SMTP server hostname
    /// - `SMTP_PORT` - SMTP port number
    /// - `SMTP_USERNAME` - SMTP authentication username (required)
    /// - `SMTP_PASSWORD` - SMTP authentication password (required)
    /// - `FROM_EMAIL` - Sender email address
    ///
    /// # Returns
    ///
    /// - `Ok(EmailService)` - Fully configured email service
    /// - `Err(EmailError)` - If SMTP credentials are missing or invalid
    ///
    /// # Errors
    ///
    /// Returns `EmailError::ConfigError` if:
    /// - SMTP credentials are not configured
    /// - Invalid SMTP port
    /// - SMTP relay connection fails
    pub fn new() -> Result<Self, EmailError> {
        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());
        let _smtp_port: u16 = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse()
            .map_err(|_| EmailError::ConfigError("Invalid SMTP port".to_string()))?;

        let smtp_username = env::var("SMTP_USERNAME").ok();
        let smtp_password = env::var("SMTP_PASSWORD").ok();

        if smtp_username.is_none() || smtp_password.is_none() {
            warn!("SMTP credentials not configured - emails will not be sent");
            return Err(EmailError::ConfigError(
                "SMTP credentials not configured".to_string(),
            ));
        }

        let creds = Credentials::new(smtp_username.unwrap(), smtp_password.unwrap());

        let mailer = SmtpTransport::relay(&smtp_host)
            .map_err(|e| EmailError::ConfigError(format!("SMTP relay error: {}", e)))?
            .credentials(creds)
            .build();

        let from_email = env::var("FROM_EMAIL")
            .unwrap_or_else(|_| "noreply@xftradesmen.com".to_string());

        Ok(EmailService { mailer, from_email })
    }

    /// Send an email asynchronously with automatic retry.
    ///
    /// This method sends an email with up to 3 automatic retries using
    /// exponential backoff. Failures are logged but this method will only
    /// fail if all retries are exhausted.
    ///
    /// # Arguments
    ///
    /// * `message` - Email message to send
    ///
    /// # Returns
    ///
    /// - `Ok(())` - Email sent successfully
    /// - `Err(EmailError)` - If all retries failed
    ///
    /// # Errors
    ///
    /// Returns `EmailError::SendError` if the message could not be sent
    /// after 3 attempts, or `EmailError::MessageError` if the message
    /// has invalid recipients or format.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use lib_core::email::{email_service, EmailMessage};
    ///
    /// let service = email_service()?;
    /// let msg = EmailMessage {
    ///     to: "user@example.com".to_string(),
    ///     subject: "Hello".to_string(),
    ///     body: "This is a test email".to_string(),
    ///     content_type: "text/plain; charset=utf8".to_string(),
    /// };
    /// service.send_email(msg).await?;
    /// ```
    pub async fn send_email(&self, message: EmailMessage) -> Result<(), EmailError> {
        let email =
            Message::builder()
                .from(
                    self.from_email.parse().map_err(|e| {
                        EmailError::MessageError(format!("Invalid from email: {}", e))
                    })?,
                )
                .to(message
                    .to
                    .parse()
                    .map_err(|e| EmailError::MessageError(format!("Invalid to email: {}", e)))?)
                .subject(message.subject)
                .header(ContentType::parse(&message.content_type).map_err(|e| {
                    EmailError::MessageError(format!("Invalid content type: {}", e))
                })?)
                .body(message.body)
                .map_err(|e| EmailError::MessageError(format!("Failed to build email: {}", e)))?;

        // Send with retry logic
        self.send_with_retry(email, 3).await
    }

    /// Send email with exponential backoff retry.
    ///
    /// Internal method that implements retry logic with exponential backoff.
    /// Each failed attempt doubles the wait time before the next retry.
    ///
    /// # Arguments
    ///
    /// * `email` - Lettre Message object to send
    /// * `max_retries` - Maximum number of attempts (including first)
    async fn send_with_retry(&self, email: Message, max_retries: u32) -> Result<(), EmailError> {
        let mut delay = std::time::Duration::from_millis(100);

        for attempt in 1..=max_retries {
            match self.mailer.send(&email) {
                Ok(_) => {
                    info!("Email sent successfully on attempt {}", attempt);
                    return Ok(());
                }
                Err(e) => {
                    error!("Email send failed on attempt {}: {}", attempt, e);

                    if attempt == max_retries {
                        return Err(EmailError::SendError(format!(
                            "Failed to send email after {} attempts: {}",
                            max_retries, e
                        )));
                    }

                    // Exponential backoff
                    tokio::time::sleep(delay).await;
                    delay = delay.saturating_mul(2);
                }
            }
        }

        unreachable!()
    }

    /// Send a contact form notification email.
    ///
    /// Sends a formatted email notification to the admin when a contact
    /// form is submitted. The notification includes the submitter's
    /// details and message content.
    ///
    /// # Arguments
    ///
    /// * `contact_name` - Name of the person submitting the form
    /// * `contact_email` - Email address of the submitter
    /// * `subject` - Optional subject line from the form
    /// * `message` - Message content from the form
    ///
    /// # Returns
    ///
    /// - `Ok(())` - Email sent successfully (after retries if needed)
    /// - `Err(EmailError)` - If email could not be sent
    ///
    /// # Configuration
    ///
    /// Sends to the email address specified in `CONTACT_NOTIFICATION_EMAIL`
    /// environment variable (defaults to admin@xftradesmen.com).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use lib_core::email::email_service;
    ///
    /// let service = email_service()?;
    /// service.send_contact_notification(
    ///     "John Doe",
    ///     "john@example.com",
    ///     Some("Website Question"),
    ///     "I have a question about your services"
    /// ).await?;
    /// ```
    pub async fn send_contact_notification(
        &self,
        contact_name: &str,
        contact_email: &str,
        subject: Option<&str>,
        message: &str,
    ) -> Result<(), EmailError> {
        let notification_email = env::var("CONTACT_NOTIFICATION_EMAIL")
            .unwrap_or_else(|_| "admin@xftradesmen.com".to_string());

        let email_subject = subject
            .map(|s| format!("Contact Form: {}", s))
            .unwrap_or_else(|| "New Contact Form Submission".to_string());

        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: linear-gradient(135deg, #d32f2f 0%, #b71c1c 100%); color: white; padding: 30px; text-align: center; border-radius: 8px 8px 0 0; }}
        .header h2 {{ margin: 0; font-size: 24px; }}
        .content {{ background: #f9f9f9; padding: 30px; border: 1px solid #ddd; }}
        .field {{ margin: 20px 0; }}
        .label {{ font-weight: bold; color: #555; margin-bottom: 5px; }}
        .value {{ color: #333; padding: 10px; background: white; border-left: 3px solid #d32f2f; padding-left: 15px; }}
        .message-box {{ margin: 20px 0; padding: 15px; background: white; border-left: 4px solid #d32f2f; }}
        .message-content {{ white-space: pre-wrap; word-wrap: break-word; color: #333; }}
        .footer {{ background: #f0f0f0; padding: 20px; text-align: center; font-size: 12px; color: #999; border-radius: 0 0 8px 8px; border: 1px solid #ddd; border-top: none; }}
        .divider {{ border: none; border-top: 1px solid #ddd; margin: 20px 0; }}
        a {{ color: #d32f2f; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h2>New Contact Form Submission</h2>
        </div>
        <div class="content">
            <div class="field">
                <div class="label">From:</div>
                <div class="value">{}</div>
            </div>
            <div class="field">
                <div class="label">Email:</div>
                <div class="value"><a href="mailto:{}">{}</a></div>
            </div>
            <hr class="divider">
            <div class="message-box">
                <div class="label">Message:</div>
                <div class="message-content">{}</div>
            </div>
        </div>
        <div class="footer">
            <p>This message was sent from your XF Tradesmen website contact form.</p>
            <p>Please reply directly to the sender's email address or use your email client's reply feature.</p>
        </div>
    </div>
</body>
</html>"#,
            contact_name,
            contact_email,
            contact_email,
            message
        );

        let email_message = EmailMessage {
            to: notification_email,
            subject: email_subject,
            body: html_body,
            content_type: "text/html; charset=utf-8".to_string(),
        };

        self.send_email(email_message).await
    }
}

/// Email service errors.
///
/// Represents various failure modes when working with the email service.
#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    /// SMTP configuration or connection error.
    ///
    /// Typically occurs when SMTP credentials are missing or invalid.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Email message building error.
    ///
    /// Occurs when the email message has invalid recipients or format.
    #[error("Message building error: {0}")]
    MessageError(String),

    /// Email sending failed after all retries.
    ///
    /// The email could not be delivered even after automatic retries.
    #[error("Send error: {0}")]
    SendError(String),
}

/// Get the global email service instance.
///
/// This function implements the singleton pattern for the email service.
/// The service is lazily initialized on first call and reused for all
/// subsequent calls. This is efficient because SMTP connections are
/// expensive to establish.
///
/// # Returns
///
/// - `Ok(EmailService)` - Configured email service ready to use
/// - `Err(EmailError)` - If SMTP credentials are not configured
///
/// # Usage
///
/// ```rust,no_run
/// use lib_core::email::email_service;
///
/// // First call initializes the service
/// let service = email_service();
///
/// match service {
///     Ok(svc) => {
///         // Service is ready to use
///         tokio::spawn(async move {
///             let _ = svc.send_contact_notification(...).await;
///         });
///     }
///     Err(e) => {
///         eprintln!("Email service not configured: {}", e);
///     }
/// }
/// ```
pub fn email_service() -> &'static Result<EmailService, EmailError> {
    static INSTANCE: std::sync::OnceLock<Result<EmailService, EmailError>> =
        std::sync::OnceLock::new();
    INSTANCE.get_or_init(|| EmailService::new())
}
