//! Blog post data module.
//!
//! Centralized data layer for blog posts with SEO-focused fields.

/// Blog post category for filtering and organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlogCategory {
    RevenueData,
    LocalSeo,
    TrustProtocols,
}

impl BlogCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RevenueData => "Revenue Data",
            Self::LocalSeo => "Local SEO",
            Self::TrustProtocols => "Trust Protocols",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            Self::RevenueData => "bg-brand/10 text-brand",
            Self::LocalSeo => "bg-blue-500/10 text-blue-400",
            Self::TrustProtocols => "bg-green-500/10 text-green-400",
        }
    }

    pub fn icon_class(&self) -> &'static str {
        match self {
            Self::RevenueData => "text-brand bg-brand/10",
            Self::LocalSeo => "text-blue-500 bg-blue-500/10",
            Self::TrustProtocols => "text-green-500 bg-green-500/10",
        }
    }
}

/// Complete blog post with all SEO fields.
#[derive(Debug, Clone)]
pub struct BlogPost {
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub category: BlogCategory,
    pub published_date: &'static str,
    pub author: &'static str,
    pub read_time: &'static str,
    pub keywords: &'static [&'static str],
    pub content: &'static str,
}

impl BlogPost {
    pub fn canonical_url(&self) -> String {
        format!("https://xftradesman.com/blog/{}", self.slug)
    }
}

/// Get all blog posts ordered by date (newest first).
pub fn get_all_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            slug: "why-tradesmen-need-websites",
            title: "Why Tradesmen with Websites Earn 40% More",
            description: "New 2024 statistics reveal the massive earnings gap between tradesmen with professional websites and those without. Learn why digital presence is no longer optional.",
            category: BlogCategory::RevenueData,
            published_date: "December 15, 2024",
            author: "XF Tradesmen",
            read_time: "5 min",
            keywords: &["tradesman website", "tradesman earnings", "website ROI", "digital presence"],
            content: r##"<p class="lead text-xl text-gray-300 mb-8 font-light leading-relaxed">
    In 2024, the data is clear: tradesmen with professional websites are earning significantly more than their competitors who rely solely on word-of-mouth. Here's why making the digital leap is no longer optional.
</p>

<h2>The Statistics Don't Lie</h2>
<p>
    According to recent industry research, 70% of homeowners now search online before hiring any tradesperson. If you don't have a website, you're invisible to this massive market segment. But it goes beyond visibility—tradesmen with websites report earning up to 40% more annually than those without.
</p>

<h3>Why the Earnings Gap Exists</h3>
<p>
    The difference isn't just about being found online. It's about the perception of professionalism and trustworthiness that a well-designed website creates. When potential clients see a quality website, they immediately assume:
</p>
<ul>
    <li><strong>Established Business:</strong> You're not a fly-by-night operator</li>
    <li><strong>Professional Standards:</strong> If you invest in your online presence, you invest in your work</li>
    <li><strong>Legitimate Operation:</strong> Easy to contact, easy to verify, easy to trust</li>
</ul>

<h2>24/7 Lead Generation</h2>
<p>
    Your website works while you sleep. Unlike traditional advertising that stops when you stop paying, your website is constantly available to capture leads. A potential customer searching for "electrician near me" at 11 PM can find you, see your work, and send an inquiry—all while you're resting for tomorrow's jobs.
</p>

<blockquote>
    "Since launching my website, I've gone from scrambling for work to having a 3-week waiting list. The quality of leads has completely changed—clients come to me already sold on my services."
    <cite>— Mark T., Electrician, Birmingham</cite>
</blockquote>

<h2>Showcase Your Best Work</h2>
<p>
    A portfolio of your completed projects is worth more than any advertisement. When potential clients can see the quality of your work through photos and testimonials, they're pre-sold before they even call.
</p>

<h2>The ROI is Undeniable</h2>
<p>
    Consider the numbers: A professional tradesman website costs a fraction of what a single new client is worth. If your average job is worth £500 and your website brings in just one new client per month, that's a 10x return on your investment in the first year alone.
</p>

<h3>Take Action Today</h3>
<p>
    The tradesmen who are winning in 2024 all have one thing in common: a professional online presence. Every day without a website is potential revenue walking straight to your competitors.
</p>"##,
        },
        BlogPost {
            slug: "local-seo-guide",
            title: "The Ultimate Guide to Local SEO for Plumbers",
            description: "Learn how to rank #1 in your local area and get your phone ringing with high-quality leads. Complete local SEO strategy for trade businesses.",
            category: BlogCategory::LocalSeo,
            published_date: "December 10, 2024",
            author: "XF Tradesmen",
            read_time: "8 min",
            keywords: &["local SEO", "plumber SEO", "Google Business Profile", "local search ranking"],
            content: r##"<p class="lead text-xl text-gray-300 mb-8 font-light leading-relaxed">
    Want to dominate your local search results and get your phone ringing with qualified leads? This comprehensive guide covers everything plumbers and tradesmen need to know about local SEO in 2024.
</p>

<h2>What is Local SEO and Why Does It Matter?</h2>
<p>
    Local SEO is the practice of optimizing your online presence to attract customers from local searches. When someone types "plumber near me" or "emergency plumber Coventry" into Google, local SEO determines whether your business shows up at the top—or gets buried on page five.
</p>

<h2>Step 1: Claim and Optimize Your Google Business Profile</h2>
<p>
    Your Google Business Profile (formerly Google My Business) is the single most important factor in local search rankings. Here's how to optimize it:
</p>
<ul>
    <li><strong>Complete Every Section:</strong> Fill out 100% of your profile—business hours, services, service areas, attributes</li>
    <li><strong>Choose the Right Categories:</strong> Select "Plumber" as your primary category, add relevant secondary categories</li>
    <li><strong>Add Quality Photos:</strong> Upload photos of your work, your van, yourself in uniform—at least 10 high-quality images</li>
    <li><strong>Write a Compelling Description:</strong> Use keywords naturally while describing your services</li>
</ul>

<h2>Step 2: Build Local Citations</h2>
<p>
    Citations are mentions of your business name, address, and phone number (NAP) on other websites. Consistency is crucial—your NAP must be identical everywhere it appears.
</p>
<h3>Priority Citation Sources for UK Tradesmen:</h3>
<ul>
    <li>Yell.com</li>
    <li>Checkatrade</li>
    <li>Trustatrader</li>
    <li>MyBuilder</li>
    <li>Thomson Local</li>
</ul>

<h2>Step 3: Generate and Manage Reviews</h2>
<p>
    Reviews are the lifeblood of local SEO. Google uses review quantity, quality, and recency as ranking factors.
</p>

<blockquote>
    "I went from 3 reviews to 47 in six months by simply asking every satisfied customer. My Google ranking went from page 2 to the top 3, and my monthly leads tripled."
    <cite>— Dave P., Plumber, Leeds</cite>
</blockquote>

<h2>Step 4: Optimize Your Website for Local Search</h2>
<p>
    Your website should reinforce your local relevance to search engines:
</p>
<ul>
    <li><strong>Location Pages:</strong> Create dedicated pages for each area you serve</li>
    <li><strong>Local Keywords:</strong> Include city and area names in titles, headings, and content</li>
    <li><strong>Schema Markup:</strong> Add LocalBusiness structured data</li>
</ul>"##,
        },
        BlogPost {
            slug: "building-trust-online",
            title: "How to Build Instant Trust with Potential Clients",
            description: "Your website is your first impression. Discover the 5 proven trust signals that convert visitors into paying customers.",
            category: BlogCategory::TrustProtocols,
            published_date: "December 5, 2024",
            author: "XF Tradesmen",
            read_time: "6 min",
            keywords: &["trust signals", "website credibility", "online reputation", "customer trust"],
            content: r##"<p class="lead text-xl text-gray-300 mb-8 font-light leading-relaxed">
    In the trades, trust is everything. Before a customer lets you into their home, they need to believe you're reliable, skilled, and honest. Here are the five proven trust signals that convert website visitors into loyal customers.
</p>

<h2>Why Online Trust Matters More Than Ever</h2>
<p>
    Research shows that 88% of consumers trust online reviews as much as personal recommendations. Your website is often the first (and sometimes only) impression potential clients have of your business.
</p>

<h2>Trust Signal #1: Professional Testimonials</h2>
<p>
    Nothing builds trust faster than hearing from satisfied customers. But not all testimonials are created equal:
</p>
<ul>
    <li><strong>Use Full Names:</strong> "John S." is good, "John Smith, Coventry" is better</li>
    <li><strong>Include Specifics:</strong> "Great work on my bathroom refit" beats "Good service"</li>
    <li><strong>Add Photos:</strong> Before/after photos with testimonials are incredibly powerful</li>
</ul>

<h2>Trust Signal #2: Credentials and Certifications</h2>
<p>
    Display your qualifications prominently. This includes:
</p>
<ul>
    <li>Trade certifications (Gas Safe, NICEIC, etc.)</li>
    <li>Professional association memberships</li>
    <li>Insurance documentation</li>
    <li>Years of experience</li>
</ul>

<blockquote>
    "The moment I added my Gas Safe registration number and insurance details prominently on my website, my conversion rate jumped 25%."
    <cite>— Steve M., Gas Engineer, Manchester</cite>
</blockquote>

<h2>Trust Signal #3: Portfolio of Completed Work</h2>
<p>
    Show, don't just tell. A gallery of your completed projects demonstrates quality, range, and consistency.
</p>

<h2>Trust Signal #4: Clear Contact Information</h2>
<p>
    Nothing screams "trustworthy" like making yourself easy to reach:
</p>
<ul>
    <li><strong>Phone Number:</strong> Prominently displayed, preferably in the header</li>
    <li><strong>Email Address:</strong> Professional email (name@yourbusiness.com)</li>
    <li><strong>Physical Address:</strong> Even if you work from home, include your service area</li>
</ul>

<h2>Trust Signal #5: Professional Website Design</h2>
<p>
    Your website design itself is a trust signal. Modern, clean layouts suggest a modern, professional business.
</p>"##,
        },
    ]
}

/// Get a single blog post by slug.
pub fn get_post_by_slug(slug: &str) -> Option<BlogPost> {
    get_all_posts().into_iter().find(|p| p.slug == slug)
}

/// Get related posts (excluding current post).
pub fn get_related_posts(current_slug: &str, limit: usize) -> Vec<BlogPost> {
    get_all_posts()
        .into_iter()
        .filter(|p| p.slug != current_slug)
        .take(limit)
        .collect()
}
