pub struct ProfileInfo<'a> {
    pub name: &'a str,
    pub title: &'a str,
}

pub const PROFILE: ProfileInfo = ProfileInfo {
    name: "Alexander Alexandrov",
    title: "Full-Stack Developer",
};

pub struct Skill<'a> {
    pub label: &'a str,
}

pub const SKILL: &[Skill] = &[
    Skill { label: "Rust" },
    Skill {
        label: "JavaScript",
    },
    Skill {
        label: "TypeScript",
    },
    Skill { label: "Java" },
    Skill { label: "React" },
    Skill { label: "Next.js" },
    Skill { label: "Redux" },
    Skill {
        label: "WebAssembly",
    },
    Skill { label: "Node.js" },
    Skill {
        label: "ExtendScript",
    },
    Skill { label: "Docker" },
    Skill {
        label: "Vercel AI SDK",
    },
    Skill { label: "AWS SDK" },
    Skill {
        label: "PostgreSQL",
    },
    Skill { label: "REST APIs" },
    Skill { label: "GraphQL" },
    Skill {
        label: "Spring MVC",
    },
    Skill { label: "Terraform" },
    Skill { label: "Git" },
    Skill { label: "macOS CLI" },
    Skill { label: "Linux CLI" },
    Skill {
        label: "LLMs (GPT, Claude)",
    },
    Skill {
        label: "JetBrains IDEs",
    },
    Skill {
        label: "Helix Editor",
    },
];

pub struct ExperienceEntry<'a> {
    pub title: &'a str,
    pub company_and_period: &'a str,
    pub responsibilities: &'a [&'a str],
}

pub const EXPERIENCE_ENTRIES: &[ExperienceEntry] = &[
    ExperienceEntry {
        title: "Full-Stack Developer",
        company_and_period: "Proxiad | Aug 2023 – Present",
        responsibilities: &[
            "Develop complex applications for a professional digital publishing solution utilizing React, TypeScript, JavaScript, ExtendScript, Redux, and SCSS.",
            "Contribute to Java back-end APIs, ensuring consistent coding standards and test coverage.",
            "Collaborate with UI/UX teams to streamline releases and improve feature turnaround time.",
        ],
    },
    ExperienceEntry {
        title: "Full-Stack Developer",
        company_and_period: "DXC Technology | Apr 2020 - Aug 2023",
        responsibilities: &[
            "Modernized an enterprise solution by migrating critical components from Java to Rust.",
            "Played a key role in developing a Visual Product Modeling System using React, TypeScript, Material UI, and Java, enhancing user interaction and product visualization capabilities.",
            "Developed a complete full-stack service using AWS SDK, React, TypeScript, and Terraform, integrating various AWS functionalities to streamline and enhance customer solutions.",
            "Developed new features for the BMW Group Vulnerability Tool; maintained PostgreSQL schemas and iBATIS data layers, resolving performance bottlenecks and reducing query time by 90% (from 30 to 3 seconds), resulting in significant cost savings and improved system efficiency.",
        ],
    },
    ExperienceEntry {
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
        description: "A multimodal, multimodel AI chatbot in React/Next.js",
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

pub const CONTACT_LINKS: &[ContactLink] = &[
    ContactLink {
        label: "Email",
        href: ContactLinkHref::Plain("mailto:contact@alexandroff.me"),
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
