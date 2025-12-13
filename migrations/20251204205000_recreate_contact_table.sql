-- Recreate Contact Form Submissions Table
-- This migration recreates the table that was accidentally dropped

CREATE TABLE IF NOT EXISTS contact_submissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    subject VARCHAR(500),
    message TEXT NOT NULL,
    submitted_at TIMESTAMP DEFAULT NOW(),
    ip_address VARCHAR(45),
    user_agent TEXT,
    CONSTRAINT email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

-- Index for faster queries by submission date
CREATE INDEX IF NOT EXISTS idx_contact_submissions_submitted_at ON contact_submissions(submitted_at DESC);

-- Index for email lookups
CREATE INDEX IF NOT EXISTS idx_contact_submissions_email ON contact_submissions(email);

-- Add comment
COMMENT ON TABLE contact_submissions IS 'Stores contact form submissions from the website';
COMMENT ON COLUMN contact_submissions.ip_address IS 'IP address of the submitter for spam prevention';
COMMENT ON COLUMN contact_submissions.user_agent IS 'Browser user agent for analytics';
