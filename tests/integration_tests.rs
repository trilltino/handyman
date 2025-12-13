//! Integration tests for main workflows
//!
//! These tests verify that the system works correctly end-to-end,
//! simulating real usage scenarios.

use lib_core::model::booking::{BookingBmc, BookingForCreate, BookingStatus, BookingForUpdate};
use lib_core::model::contact::{ContactBmc, ContactForCreate};
use lib_core::model::testimonial::{TestimonialBmc, TestimonialForCreate};
use lib_core::_dev_utils;
use time::Duration;

#[tokio::test]
async fn test_workflow_booking_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup
    let mm = _dev_utils::init_test().await;
    let customer_id = 1000;
    let tradesman_id = 2000;

    // 2. Create Booking
    let booking_create = BookingForCreate {
        customer_id,
        tradesman_id,
        service_type: "Integration Test Service".to_string(),
        description: "Full lifecycle test".to_string(),
        scheduled_date: time::PrimitiveDateTime::new(
            time::OffsetDateTime::now_utc().date(),
            time::Time::from_hms(10, 0, 0).unwrap()
        ),
        location: "Test Location".to_string(),
        estimated_duration_hours: Some(4),
        price_quote: Some(200.0),
    };
    let booking_id = BookingBmc::create(&mm, booking_create).await?;
    assert!(!booking_id.is_nil());

    // 3. Verify Initial State
    let booking = BookingBmc::get_by_id(&mm, booking_id).await?.unwrap();
    assert_eq!(booking.status, BookingStatus::Pending);

    // 4. Confirm Booking
    let update_confirm = BookingForUpdate {
        status: Some(BookingStatus::Confirmed),
        ..Default::default()
    };
    BookingBmc::update(&mm, booking_id, update_confirm).await?;

    let booking = BookingBmc::get_by_id(&mm, booking_id).await?.unwrap();
    assert_eq!(booking.status, BookingStatus::Confirmed);

    // 5. Complete Booking
    let update_complete = BookingForUpdate {
        status: Some(BookingStatus::Completed),
        ..Default::default()
    };
    BookingBmc::update(&mm, booking_id, update_complete).await?;

    let booking = BookingBmc::get_by_id(&mm, booking_id).await?.unwrap();
    assert_eq!(booking.status, BookingStatus::Completed);

    // 6. Leave Testimonial
    let testimonial_create = TestimonialForCreate {
        customer_id,
        tradesman_id,
        booking_id: Some(booking_id),
        rating: 5,
        title: "Great Job".to_string(),
        content: "Completed on time".to_string(),
    };
    let testimonial_id = TestimonialBmc::create(&mm, testimonial_create).await?;
    assert!(!testimonial_id.is_nil());

    // 7. Verify Testimonial Linked
    let testimonial = TestimonialBmc::get_by_id(&mm, testimonial_id).await?.unwrap();
    assert_eq!(testimonial.booking_id, Some(booking_id));
    assert_eq!(testimonial.rating, 5);

    // 8. Cleanup
    TestimonialBmc::delete(&mm, testimonial_id).await?;
    BookingBmc::delete(&mm, booking_id).await?;

    Ok(())
}

#[tokio::test]
async fn test_workflow_contact_submission() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup
    let mm = _dev_utils::init_test().await;

    // 2. Submit Contact Form
    let contact_create = ContactForCreate {
        name: "Integration User".to_string(),
        email: "integration@example.com".to_string(),
        message: "Testing contact workflow".to_string(),
        subject: Some("Integration Test".to_string()),
        ip_address: Some("127.0.0.1".to_string()),
        user_agent: Some("TestRunner/1.0".to_string()),
    };
    let contact_id = ContactBmc::create(&mm, contact_create).await?;
    assert!(contact_id > 0);

    // 3. Verify Submission
    let contact = ContactBmc::get(&mm, contact_id).await?;
    assert_eq!(contact.email, "integration@example.com");

    // 4. List Submissions
    let list = ContactBmc::list(&mm).await?;
    assert!(list.iter().any(|c| c.id == contact_id));

    // 5. Cleanup
    ContactBmc::delete(&mm, contact_id).await?;

    Ok(())
}
