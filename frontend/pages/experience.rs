//! # Experience Page
//!
//! ## Purpose
//! Showcase handyman experience, skills, and portfolio.
//! Builds trust by displaying qualifications and past work.
//!
//! ## Content
//! - Professional experience timeline
//! - Skills and certifications
//! - Work examples/portfolio
//! - Testimonials (future)
//!
//! ## Relation to Entire Program
//! - **Route**: `/experience`
//! - **Public**: No authentication required
//! - **Goal**: Build credibility and convert visitors to customers

use yew::prelude::*;  // Yew framework
use stylist::css;     // CSS-in-Rust styling

#[derive(Clone, PartialEq)]
struct ExperienceItem {
    title: &'static str,
    company: &'static str,
    period: &'static str,
    description: &'static str,
    technologies: Vec<&'static str>,
}

#[function_component]
pub fn Experience() -> Html {
    let css = css!(r#"
        .experience-container { max-width: 900px; margin: 50px auto; padding: 20px; }
        .experience-container h1 { text-align: center; color: #2c3e50; margin-bottom: 40px; }
        .timeline { position: relative; }
        .experience-item { background: #fff; padding: 30px; margin-bottom: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); border-left: 4px solid #3498db; }
        .experience-header { margin-bottom: 15px; }
        .experience-title { color: #2c3e50; font-size: 1.5rem; margin-bottom: 5px; }
        .experience-company { color: #3498db; font-size: 1.2rem; margin-bottom: 5px; }
        .experience-period { color: #7f8c8d; font-style: italic; }
        .experience-description { color: #34495e; line-height: 1.6; margin-bottom: 15px; }
        .technologies { display: flex; flex-wrap: wrap; gap: 10px; }
        .tech-tag { background: #ecf0f1; color: #2c3e50; padding: 5px 12px; border-radius: 15px; font-size: 0.9rem; }
    "#);

    let experiences = vec![
        ExperienceItem {
            title: "Senior Full Stack Developer",
            company: "Tech Innovations Inc.",
            period: "2023 - Present",
            description: "Leading development of scalable web applications using modern Rust frameworks. Architected and implemented microservices infrastructure serving millions of users.",
            technologies: vec!["Rust", "Axum", "Yew", "PostgreSQL", "Docker", "Kubernetes"],
        },
        ExperienceItem {
            title: "Backend Engineer",
            company: "StartUp Solutions",
            period: "2021 - 2023",
            description: "Developed high-performance backend services and APIs. Optimized database queries reducing response times by 60%. Implemented comprehensive testing strategies.",
            technologies: vec!["Rust", "Actix", "MongoDB", "Redis", "GraphQL"],
        },
        ExperienceItem {
            title: "Software Developer",
            company: "Digital Creations",
            period: "2019 - 2021",
            description: "Built responsive web applications and contributed to open-source projects. Collaborated with cross-functional teams to deliver customer-focused solutions.",
            technologies: vec!["JavaScript", "React", "Node.js", "MySQL", "AWS"],
        },
    ];

    html! {
        <div class={css}>
            <div class="experience-container">
                <h1>{ "Professional Experience" }</h1>
                <div class="timeline">
                    {
                        experiences.iter().map(|exp| {
                            html! {
                                <div class="experience-item">
                                    <div class="experience-header">
                                        <h2 class="experience-title">{ exp.title }</h2>
                                        <h3 class="experience-company">{ exp.company }</h3>
                                        <p class="experience-period">{ exp.period }</p>
                                    </div>
                                    <p class="experience-description">{ exp.description }</p>
                                    <div class="technologies">
                                        {
                                            exp.technologies.iter().map(|tech| {
                                                html! { <span class="tech-tag">{ tech }</span> }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </div>
    }
}
