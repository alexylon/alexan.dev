pub struct ProfileInfo<'a> {
    pub name: &'a str,
    pub title: &'a str,
}

pub enum ContactLinkHref<'a> {
    Plain(&'a str),
    ResumeAsset,
}

pub struct ContactLink<'a> {
    pub label: &'a str,
    pub href: ContactLinkHref<'a>,
    pub target: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub download: Option<&'a str>,
}

pub const PROFILE: ProfileInfo = ProfileInfo {
    name: "Alexander Alexandrov",
    title: "Full-Stack Developer",
};

pub const CONTACT_LINKS: &[ContactLink] = &[
    ContactLink {
        label: "Sofia",
        href: ContactLinkHref::Plain(
            "https://www.google.com/maps/place/42°41'05.7\"N+23°19'08.5\"E",
        ),
        target: Some("_blank"),
        rel: Some("noopener noreferrer"),
        download: None,
    },
    ContactLink {
        label: "Email",
        href: ContactLinkHref::Plain("mailto:contact@alexandrov.cc"),
        target: None,
        rel: None,
        download: None,
    },
    ContactLink {
        label: "LinkedIn",
        href: ContactLinkHref::Plain("https://www.linkedin.com/in/alexandrovalexander/"),
        target: Some("_blank"),
        rel: Some("noopener noreferrer"),
        download: None,
    },
    ContactLink {
        label: "GitHub",
        href: ContactLinkHref::Plain("https://github.com/alexylon"),
        target: Some("_blank"),
        rel: Some("noopener noreferrer"),
        download: None,
    },
    // ContactLink {
    //     label: "Resume ⬇",
    //     href: ContactLinkHref::ResumeAsset,
    //     target: None,
    //     rel: None,
    //     download: Some("resume_alexander_alexandrov.pdf"),
    // },
];

pub struct Skill<'a> {
    pub label: &'a str,
}

pub const SKILL: &[Skill] = &[
    Skill { label: "Rust" },
    Skill { label: "JavaScript/TypeScript" },
    Skill { label: "Java" },
    Skill { label: "React" },
    Skill { label: "Next.js" },
    Skill { label: "Redux" },
    Skill { label: "WebAssembly" },
    Skill { label: "AWS (SDK)" },
    Skill { label: "PostgreSQL" },
    Skill { label: "REST/GraphQL" },
    Skill { label: "Spring MVC" },
    Skill { label: "Terraform" },
    Skill { label: "Docker" },
    Skill { label: "Git" },
    Skill { label: "Linux/macOS CLI" },
    Skill { label: "Vercel AI SDK" },
];

pub struct CareerEntry<'a> {
    pub title: &'a str,
    pub company_and_period: &'a str,
    pub responsibilities: &'a [&'a str],
}

pub const CAREER_ENTRIES: &[CareerEntry] = &[
    CareerEntry {
        title: "Full-Stack Developer",
        company_and_period: "Proxiad | Aug 2023 – Present",
        responsibilities: &[
            "Deliver complex applications with React/Next.js, TypeScript, and SCSS.",
            "Contribute to Java back-end APIs, ensuring consistent coding standards and test coverage.",
            "Collaborate with UI/UX teams to streamline releases and improve feature turnaround time.",
        ],
    },
    CareerEntry {
        title: "Full-Stack Developer",
        company_and_period: "DXC Technology | Apr 2020 - Aug 2023",
        responsibilities: &[
            "Modernized an enterprise solution by migrating critical components from Java to Rust.",
            "Built front-end features in React and Material UI; implemented REST services with Spring MVC.",
            "Automated cloud infrastructure with Terraform and AWS services, improving deployment reliability.",
            "Maintained PostgreSQL schemas and iBATIS data layers, resolving performance bottlenecks and reducing query time by 90%.",
        ],
    },
    CareerEntry {
        title: "Web Developer Intern",
        company_and_period: "Camplight | Sep 2019 - Apr 2020",
        responsibilities: &[
            "Developed a web application using React, Next.js, and GraphQL.",
            "Wrote reusable UI components in TypeScript and Material UI.",
            "Collaborated closely with mentors to review architecture and adhere to best coding practices, ensuring production-ready quality.",
        ],
    },
];

pub struct ProjectEntry<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub url: &'a str,
}

pub const PROJECTS: &[ProjectEntry] = &[
    ProjectEntry {
        name: "Ferrocrypt",
        description: "A gui/cli encryption/decryption tool in Rust",
        url: "https://github.com/alexylon/Ferrocrypt",
    },
    ProjectEntry {
        name: "Sofos",
        description: "A multimodal AI chatbot in React/Next.js",
        url: "https://github.com/alexylon/Sofos",
    },
    ProjectEntry {
        name: "Wordle",
        description: "The popular Wordle game in Rust",
        url: "https://github.com/alexylon/wordle",
    },
    ProjectEntry {
        name: "Webpublication",
        description: "Professional digital publishing solution",
        url: "https://webpublication.co.uk/",
    },
    ProjectEntry {
        name: "VP/MS",
        description: "Visual Product Modeling System",
        url: "https://en.wikipedia.org/wiki/VP/MS",
    },
    ProjectEntry {
        name: "Vulnerability Tool",
        description: "The BMW Group vulnerability tool",
        url: "https://www.bmwgroup.com/en/general/Security.html",
    },
];

pub struct EducationEntry<'a> {
    pub title: &'a str,
    pub institution: &'a str,
}

pub const EDUCATION: &[EducationEntry] = &[
    EducationEntry {
        title: "Java Fundamentals — ",
        institution: "MaxPlus",
    },
    EducationEntry {
        title: "React & JavaScript — ",
        institution: "Camplight",
    },
    EducationEntry {
        title: "M.Eng., Engineering — ",
        institution: "University of Forestry",
    },
    EducationEntry {
        title: "PhD, Theology — ",
        institution: "Sofia University “St. Kliment Ohridski”",
    },
    EducationEntry {
        title: "Mathematics — ",
        institution: "High School of Mathematics",
    },
];

pub struct Language<'a> {
    pub name: &'a str,
    pub level: &'a str,
}

pub const LANGUAGES: &[Language] = &[
    Language {
        name: "Bulgarian",
        level: "(Native)",
    },
    Language {
        name: "English",
        level: "(C1)",
    },
    Language {
        name: "Italian",
        level: "(B2)",
    },
    Language {
        name: "Russian",
        level: "(B1)",
    },
    Language {
        name: "Greek",
        level: "(A2)",
    },
];
