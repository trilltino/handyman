-- Blog Posts Table
-- Stores blog posts for the website

CREATE TYPE blog_post_status AS ENUM ('draft', 'published', 'archived');

CREATE TABLE IF NOT EXISTS blog_posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    excerpt TEXT,
    author VARCHAR(100) NOT NULL,
    status blog_post_status DEFAULT 'draft',
    seo_title VARCHAR(255),
    seo_description TEXT,
    featured_image VARCHAR(500),
    tags VARCHAR(500),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    published_at TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_blog_posts_slug ON blog_posts(slug);
CREATE INDEX IF NOT EXISTS idx_blog_posts_status ON blog_posts(status);
CREATE INDEX IF NOT EXISTS idx_blog_posts_published_at ON blog_posts(published_at DESC);
CREATE INDEX IF NOT EXISTS idx_blog_posts_author ON blog_posts(author);

-- Add comments
COMMENT ON TABLE blog_posts IS 'Stores blog posts for the website';
COMMENT ON COLUMN blog_posts.slug IS 'URL-friendly identifier for the post';
COMMENT ON COLUMN blog_posts.status IS 'Publication status: draft, published, or archived';
COMMENT ON COLUMN blog_posts.published_at IS 'Timestamp when post was published (null for drafts)';

-- Newsletter Subscribers Table
-- Stores email subscribers for newsletter

CREATE TABLE IF NOT EXISTS newsletter_subscribers (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    subscribed_at TIMESTAMP DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);

-- Index for email lookups
CREATE INDEX IF NOT EXISTS idx_newsletter_subscribers_email ON newsletter_subscribers(email);
CREATE INDEX IF NOT EXISTS idx_newsletter_subscribers_active ON newsletter_subscribers(is_active);

-- Add comment
COMMENT ON TABLE newsletter_subscribers IS 'Stores newsletter email subscribers';

-- Down migration
-- DROP TABLE IF EXISTS newsletter_subscribers;
-- DROP TABLE IF EXISTS blog_posts;
-- DROP TYPE IF EXISTS blog_post_status;