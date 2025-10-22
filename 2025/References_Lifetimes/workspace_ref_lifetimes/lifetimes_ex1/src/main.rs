/*
 * Example: build a ResearchPaper struct that borrows:
 * - a title
 * - a lead scientist
 * - and a main formula or theory name.
 *
 * Store these structs to `ScientificArchive` and write a function to compare which paper
 * references a longer formula or theory.
 *
 */

#[allow(dead_code)]
#[derive(Debug)]
struct ResearchPaper<'a> {
    title: &'a str,
    lead_scientist: &'a str,
    formula: &'a str,
}

#[derive(Debug)]
struct ScientificArchive<'a> {
    papers: Vec<ResearchPaper<'a>>,
}

impl<'a> ScientificArchive<'a> {
    fn new() -> Self {
        ScientificArchive { papers: Vec::new() }
    }

    fn add_paper(&mut self, title: &'a str, lead_scientist: &'a str, formula: &'a str) {
        let paper = ResearchPaper {
            title,
            lead_scientist,
            formula,
        };
        self.papers.push(paper);
    }

    // Return the paper that references the longer formula
    fn more_complex_paper<'b>(
        &'b self,
        p1: &'b ResearchPaper<'a>,
        p2: &'b ResearchPaper<'a>,
    ) -> &'b ResearchPaper<'a> {
        if p1.formula.len() >= p2.formula.len() {
            p1
        } else {
            p2
        }
    }
}

fn main() {
    let title1 = String::from("Quantum Field Interactions");
    let lead1 = String::from("Dr. Marie Curie");
    let formula1 = String::from("L = ∫ d⁴x ℒ");

    let title2 = String::from("General Relativity Explained");
    let lead2 = String::from("Dr. Albert Einstein");
    let formula2 = String::from("R_{μν} - ½g_{μν}R + Λg_{μν} = 8πGT_{μν}");

    // All strings must outlive their borrowed use in the archive
    let mut archive = ScientificArchive::new();

    archive.add_paper(&title1, &lead1, &formula1);
    archive.add_paper(&title2, &lead2, &formula2);

    let paper1 = &archive.papers[0];
    let paper2 = &archive.papers[1];

    let complex_paper = archive.more_complex_paper(paper1, paper2);

    println!("Paper with the more complex formula: {:?}", complex_paper);
}
