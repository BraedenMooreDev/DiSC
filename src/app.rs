use std::{fmt::Debug, f64::consts::E, ops::RangeInclusive};
use eframe::epaint::RectShape;
use egui::{Vec2, FontId, TextStyle, Ui, Context, RichText, Color32, Style, Rect, Shape, Sense, plot::{Plot, Points, PlotPoints, PlotPoint, Line, PlotBounds, GridMark, GridInput, BarChart, Bar}, accesskit::Point, Pos2};


#[derive(PartialEq, Clone, Copy, Debug)]
enum Choice { A = 0, B = 1, C = 2, D = 3, E = 4, NONE = 5}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Page { Response = 0, Results = 1, Settings = 2}

#[derive(PartialEq, Clone, Copy, Debug)]
enum GraphType { Line = 0, Bar = 1 }

#[derive(PartialEq, Clone, Debug)]
struct Profile {
    name: String,
    aspects: Vec<(String, String)>,
    content: String
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:

    fontSizes: (f32, f32, f32, f32, f32), // Heading, Body, Monospace, Button, Small

    #[serde(skip)]
    currentPage: Page,
    
    #[serde(skip)]
    currentHighlight: Choice,

    #[serde(skip)]
    graphType: GraphType,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    questions: Vec<Vec<(String, Choice, Choice)>>,
    
    #[serde(skip)]
    responses: Vec<(Choice, Choice)>,

    #[serde(skip)]
    tally: (i8, i8, i8, i8),

    #[serde(skip)]
    intensity: (i8, i8, i8, i8),

    #[serde(skip)]
    profiles: Vec<Profile>,

    #[serde(skip)]
    currentProfile: Profile
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            fontSizes: (30.0, 18.0, 14.0, 14.0, 10.0),
            currentPage: Page::Response,
            currentHighlight: Choice::NONE,
            graphType: GraphType::Line,
            questions: vec![
                vec![("enthusiastic".to_string(), Choice::B, Choice::B),
                    ("daring".to_string(), Choice::A, Choice::A),
                    ("diplomatic".to_string(), Choice::D, Choice::D),
                    ("satisfied".to_string(), Choice::C, Choice::C)],

                vec![("cautious".to_string(), Choice::D, Choice::D),
                    ("determined".to_string(), Choice::A, Choice::A),
                    ("convincing".to_string(), Choice::B, Choice::B),
                    ("good-natured".to_string(), Choice::C, Choice::E)],
                    
                vec![("friendly".to_string(), Choice::B, Choice::E),
                    ("accurate".to_string(), Choice::D, Choice::D),
                    ("outspoken".to_string(), Choice::A, Choice::A),
                    ("calm".to_string(), Choice::E, Choice::C)],
                
                vec![("talkative".to_string(), Choice::B, Choice::B),
                    ("controlled".to_string(), Choice::D, Choice::D),
                    ("conventional".to_string(), Choice::C, Choice::C),
                    ("decisive".to_string(), Choice::A, Choice::A)],
                    
                vec![("adventurous".to_string(), Choice::A, Choice::A),
                    ("insightful".to_string(), Choice::D, Choice::D),
                    ("outgoing".to_string(), Choice::B, Choice::B),
                    ("moderate".to_string(), Choice::C, Choice::C)],
                    
                vec![("gentle".to_string(), Choice::C, Choice::C),
                    ("persuasive".to_string(), Choice::B, Choice::E),
                    ("humble".to_string(), Choice::E, Choice::D),
                    ("original".to_string(), Choice::NONE, Choice::A)],
                
                vec![("expressive".to_string(), Choice::B, Choice::B),
                    ("conscientious".to_string(), Choice::D, Choice::D),
                    ("dominant".to_string(), Choice::A, Choice::A),
                    ("responsive".to_string(), Choice::E, Choice::C)], 
                    
                vec![("poised".to_string(), Choice::B, Choice::B),
                    ("observant".to_string(), Choice::D, Choice::E),
                    ("modest".to_string(), Choice::C, Choice::C),
                    ("impatient".to_string(), Choice::A, Choice::A)],
                    
                vec![("tactful".to_string(), Choice::D, Choice::D),
                    ("agreeable".to_string(), Choice::C, Choice::C),
                    ("magnetic".to_string(), Choice::B, Choice::B),
                    ("insistent".to_string(), Choice::A, Choice::A)],
                    
                vec![("brave".to_string(), Choice::A, Choice::A),
                    ("inspiring".to_string(), Choice::B, Choice::B),
                    ("submissive".to_string(), Choice::C, Choice::C),
                    ("timid".to_string(), Choice::E, Choice::D)],
                    
                vec![("reserved".to_string(), Choice::D, Choice::D),
                    ("obliging".to_string(), Choice::C, Choice::C),
                    ("strong-willed".to_string(), Choice::A, Choice::A),
                    ("cheerful".to_string(), Choice::B, Choice::B)],
                    
                vec![("stimulating".to_string(), Choice::B, Choice::B),
                    ("kind".to_string(), Choice::C, Choice::C),
                    ("perceptive".to_string(), Choice::D, Choice::D),
                    ("independent".to_string(), Choice::A, Choice::A)],
                    
                vec![("competitive".to_string(), Choice::A, Choice::A),
                    ("considerate".to_string(), Choice::C, Choice::C),
                    ("joyful".to_string(), Choice::B, Choice::B),
                    ("private".to_string(), Choice::D, Choice::D)],
                    
                vec![("fussy".to_string(), Choice::D, Choice::D),
                    ("obedient".to_string(), Choice::C, Choice::C),
                    ("firm".to_string(), Choice::A, Choice::A),
                    ("playful".to_string(), Choice::B, Choice::B)],
                    
                vec![("attractive".to_string(), Choice::B, Choice::B),
                    ("introspective".to_string(), Choice::D, Choice::E),
                    ("stubborn".to_string(), Choice::A, Choice::A),
                    ("predictable".to_string(), Choice::C, Choice::C)],
                    
                vec![("logical".to_string(), Choice::D, Choice::D),
                    ("bold".to_string(), Choice::A, Choice::A),
                    ("loyal".to_string(), Choice::C, Choice::C),
                    ("charming".to_string(), Choice::B, Choice::B)],
                    
                vec![("sociable".to_string(), Choice::B, Choice::B),
                    ("patient".to_string(), Choice::C, Choice::C),
                    ("self-reliant".to_string(), Choice::A, Choice::A),
                    ("soft-spoken".to_string(), Choice::D, Choice::D)],
                    
                vec![("willing".to_string(), Choice::C, Choice::C),
                    ("eager".to_string(), Choice::A, Choice::E),
                    ("thorough".to_string(), Choice::D, Choice::D),
                    ("high-spririted".to_string(), Choice::B, Choice::B)],
                    
                vec![("aggressive".to_string(), Choice::A, Choice::A),
                    ("extroverted".to_string(), Choice::B, Choice::B),
                    ("amiable".to_string(), Choice::C, Choice::C),
                    ("fearful".to_string(), Choice::E, Choice::D)],
                    
                vec![("confident".to_string(), Choice::B, Choice::B),
                    ("sympathetic".to_string(), Choice::C, Choice::C),
                    ("impartial".to_string(), Choice::E, Choice::D),
                    ("assertive".to_string(), Choice::A, Choice::A)],
                    
                vec![("well-disciplined".to_string(), Choice::D, Choice::D),
                    ("generous".to_string(), Choice::C, Choice::C),
                    ("animated".to_string(), Choice::B, Choice::B),
                    ("persistent".to_string(), Choice::A, Choice::A)],
                    
                vec![("impulsive".to_string(), Choice::B, Choice::B),
                    ("introverted".to_string(), Choice::D, Choice::D),
                    ("forceful".to_string(), Choice::A, Choice::A),
                    ("easygoing".to_string(), Choice::C, Choice::C)],
                    
                vec![("good mixer".to_string(), Choice::B, Choice::B),
                    ("refined".to_string(), Choice::D, Choice::D),
                    ("vigorous".to_string(), Choice::A, Choice::A),
                    ("lenient".to_string(), Choice::C, Choice::C)],
                    
                vec![("captivating".to_string(), Choice::B, Choice::B),
                    ("contented".to_string(), Choice::C, Choice::C),
                    ("demanding".to_string(), Choice::A, Choice::A),
                    ("compliant".to_string(), Choice::D, Choice::D)],
                    
                vec![("argumentative".to_string(), Choice::A, Choice::A),
                    ("systematic".to_string(), Choice::D, Choice::D),
                    ("cooperative".to_string(), Choice::C, Choice::C),
                    ("light-hearted".to_string(), Choice::B, Choice::B)],
                    
                vec![("jovial".to_string(), Choice::B, Choice::B),
                    ("precise".to_string(), Choice::D, Choice::D),
                    ("direct".to_string(), Choice::A, Choice::A),
                    ("even-tempered".to_string(), Choice::C, Choice::C)],
                    
                vec![("restless".to_string(), Choice::A, Choice::A),
                    ("neighborly".to_string(), Choice::C, Choice::C),
                    ("appealing".to_string(), Choice::B, Choice::B),
                    ("careful".to_string(), Choice::D, Choice::D)],
                    
                vec![("respectful".to_string(), Choice::D, Choice::D),
                    ("pioneering".to_string(), Choice::A, Choice::A),
                    ("optimistic".to_string(), Choice::B, Choice::B),
                    ("helpful".to_string(), Choice::C, Choice::C)]],

            responses: vec![(Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE),
                            (Choice::NONE, Choice::NONE)],

            tally: (0, 0, 0, 0),
            intensity: (1, 1, 1, 1),

            profiles: vec![Profile {name: "Achiever".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "is industrious and diligent; displays frustration".to_owned()),
                                                    ("Goal:".to_owned(), "personal accomplishments, sometimes at the expense of the group's goal".to_owned()),
                                                    ("Judges others by:".to_owned(), "ability to achieve concrete results".to_owned()),
                                                    ("Influences others by:".to_owned(), "accountability for own work".to_owned()),
                                                    ("Value to the organization:".to_owned(), "sets and completes key result areas for self".to_owned()),
                                                    ("Overuses:".to_owned(), "self-reliance; absorption in the task".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes frustrated and impatient; becomes more of a\"do-er\" and less of a \"delegator\"".to_owned()),
                                                    ("Fears".to_owned(), "others with competing or inferior work standards affecting results".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "less \"either-or\" thinking; cleaner task priorities; consideration of optional approaches; willingness to compromise short-term for long-range benefits".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Agent".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "accepts affection; rejects aggression".to_owned()),
                                                    ("Goal:".to_owned(), "group acceptance".to_owned()),
                                                    ("Judges others by:".to_owned(), "commitment to tolerate and include everyone".to_owned()),
                                                    ("Influences others by:".to_owned(), "empathy; friendship".to_owned()),
                                                    ("Value to the organization:".to_owned(), "supports, harmonizes, empathizes; focuses on service".to_owned()),
                                                    ("Overuses:".to_owned(), "kindness".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes persuasive, using information or key friendships if necessary".to_owned()),
                                                    ("Fears".to_owned(), "dissent; conflict".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "strength in the realization of who they are and what they can do; firmness and self-assertion; ability to say \"no\" when appropriate".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Appraiser".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "is driven to look good".to_owned()),
                                                    ("Goal:".to_owned(), "\"victory\" with flair".to_owned()),
                                                    ("Judges others by:".to_owned(), "ability to initiate activities".to_owned()),
                                                    ("Influences others by:".to_owned(), "competitive recognition".to_owned()),
                                                    ("Value to the organization:".to_owned(), "accomplishes goals with the team".to_owned()),
                                                    ("Overuses:".to_owned(), "authority; ingenuity".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes restless, critical, impatient".to_owned()),
                                                    ("Fears".to_owned(), "\"loss\" or \"failure\"; others' disapproval".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "individual follow-through; empathy when showing disapproval; steadier pace".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Counselor".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "is approachable; shows affection and understanding".to_owned()),
                                                    ("Goal:".to_owned(), "friendship; happiness".to_owned()),
                                                    ("Judges others by:".to_owned(), "positive acceptance or others; ability to look for the good in people".to_owned()),
                                                    ("Influences others by:".to_owned(), "personal relationships; \"open door\" policy".to_owned()),
                                                    ("Value to the organization:".to_owned(), "remains stable and predictable; develops a wide range of friendships; listens to others' feelings".to_owned()),
                                                    ("Overuses:".to_owned(), "indirect approach; tolerance".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes overly flexible and intimate; is too trusting without differentiating among people".to_owned()),
                                                    ("Fears".to_owned(), "pressuring people; being accused of causing harm".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "attention to realistic deadlines; initiative to complete the task".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Creative".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "accepts aggression; restrains expression".to_owned()),
                                                    ("Goal:".to_owned(), "dominance; unique accomplishments".to_owned()),
                                                    ("Judges others by:".to_owned(), "personal standards; progressive ideas for accomplishing tasks".to_owned()),
                                                    ("Influences others by:".to_owned(), "ability to pace development of systems and innovative approaches".to_owned()),
                                                    ("Value to the organization:".to_owned(), "initiates or designs change".to_owned()),
                                                    ("Overuses:".to_owned(), "bluntness; critical or condescending attitude".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes bored with routine work; sulks when restrained; acts independently".to_owned()),
                                                    ("Fears".to_owned(), "lack of influence; failure to achieve their standards".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "warmth; tactful communication; effective team cooperation; recognition of existing sanctions".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Developer".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "is concerned with meeting personal needs".to_owned()),
                                                    ("Goal:".to_owned(), "new opportunities".to_owned()),
                                                    ("Judges others by:".to_owned(), "ability to meet the Developer's standards".to_owned()),
                                                    ("Influences others by:".to_owned(), "finding solutions to problems; projecting a personal sense of power".to_owned()),
                                                    ("Value to the organization:".to_owned(), "avoids \"passing the buck\"; seeks new or innovative problem-solving methods".to_owned()),
                                                    ("Overuses:".to_owned(), "control over people and situations to accomplish his or her own results".to_owned()),
                                                    ("Under pressure:".to_owned(), "works alone to complete tasks; is belligerant if individualism is threatened or challenging opportunities disappear".to_owned()),
                                                    ("Fears".to_owned(), "boredom; loss of control".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "patience, empathy; participation and collaboration with others; follow-through and attention to quality control".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Inspirational".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "accepts aggression; downplays need for affection".to_owned()),
                                                    ("Goal:".to_owned(), "control of their environment or audience".to_owned()),
                                                    ("Judges others by:".to_owned(), "projection of personal strength, character, and social power".to_owned()),
                                                    ("Influences others by:".to_owned(), "charm, direction, intimidation; use of rewards".to_owned()),
                                                    ("Value to the organization:".to_owned(), "acts as a \"people mover\"; initiates, demands, compliments, disciplines".to_owned()),
                                                    ("Overuses:".to_owned(), "attitude that \"the ends justify the means\"".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes manipulative, quarrelsome or belligerant".to_owned()),
                                                    ("Fears".to_owned(), "weak behavior; loss of social status".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "genuine sensitivity; willingness to help others to succeed in their own personal development".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Investigator".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "is dispassionate; demonstrates self-discipline".to_owned()),
                                                    ("Goal:".to_owned(), "power through formal roles and positions of authority".to_owned()),
                                                    ("Judges others by:".to_owned(), "use of factual information".to_owned()),
                                                    ("Influences others by:".to_owned(), "determination, tenacity".to_owned()),
                                                    ("Value to the organization:".to_owned(), "offers comprehensive follow-through; works determinedly on tasks individuall or in a small group".to_owned()),
                                                    ("Overuses:".to_owned(), "bluntness; suspicion of others".to_owned()),
                                                    ("Under pressure:".to_owned(), "tends to internalize conflict; holds on to grudges".to_owned()),
                                                    ("Fears".to_owned(), "involvement with the masses; responsibility to sell abstract ideas".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "flexibility; acceptance or others; personal involvement with others".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Objective Thinker".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "rejects interpersonal aggression".to_owned()),
                                                    ("Goal:".to_owned(), "correctness".to_owned()),
                                                    ("Judges others by:".to_owned(), "ability to think logically".to_owned()),
                                                    ("Influences others by:".to_owned(), "use of facts, data, and logical arguments".to_owned()),
                                                    ("Value to the organization:".to_owned(), "defines and clarifies; obtains, evaluates, and tests information".to_owned()),
                                                    ("Overuses:".to_owned(), "analysis".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes worrisome".to_owned()),
                                                    ("Fears".to_owned(), "irrational acts; ridicule".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "self-disclosure; public discussion of their insights and opinions".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Perfectionist".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "displays competence; is restrained and cautious".to_owned()),
                                                    ("Goal:".to_owned(), "stability; predictble accomplishments".to_owned()),
                                                    ("Judges others by:".to_owned(), "precise standards".to_owned()),
                                                    ("Influences others by:".to_owned(), "attention to detail; accuracy".to_owned()),
                                                    ("Value to the organization:".to_owned(), "is conscientious; maintains standards; controls quality".to_owned()),
                                                    ("Overuses:".to_owned(), "procedures and \"fail-safe\" controls; overdependence on people, products, and processes that have worked in past".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes tactful and diplomatic".to_owned()),
                                                    ("Fears".to_owned(), "antogonism".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "role flexibility; independence and interdependence; belief in self-worth".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Persuader".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "trusts others; is enthusiastic".to_owned()),
                                                    ("Goal:".to_owned(), "authority and prestige; status symbols".to_owned()),
                                                    ("Judges others by:".to_owned(), "ability to express themselves; flexibility".to_owned()),
                                                    ("Influences others by:".to_owned(), "friendly, open manner; verbal skills".to_owned()),
                                                    ("Value to the organization:".to_owned(), "sells and closes; delegates responsibility; is poised and confident".to_owned()),
                                                    ("Overuses:".to_owned(), "enthusiasm; selling ability; optimism".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes indecisive and is easily persuaded; becomes organized in order to look good".to_owned()),
                                                    ("Fears".to_owned(), "fixed environment; complex relationships".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "challenging assignments; attention to task-oriented service and key details; objective data analysis".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Practitioner".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "wants to keep up with others in effort and technical performance".to_owned()),
                                                    ("Goal:".to_owned(), "personal growth".to_owned()),
                                                    ("Judges others by:".to_owned(), "self-discipline; position and promotions".to_owned()),
                                                    ("Influences others by:".to_owned(), "confidence in their ability to master new skills; development of \"proper\" procedures and actions".to_owned()),
                                                    ("Value to the organization:".to_owned(), "is skilled in technical and people problem-solving; displays proficiency and specialization".to_owned()),
                                                    ("Overuses:".to_owned(), "overattention to personal objectives; unrealistic expectations of others".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes restrained; is sensitive to criticism".to_owned()),
                                                    ("Fears".to_owned(), "being too predictable; no recognition as an \"expert\"".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "genuine collaboration for common benefit; delegation of key tasks to appropriate individuals".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Promoter".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "is willing to accept others".to_owned()),
                                                    ("Goal:".to_owned(), "approval, popularity".to_owned()),
                                                    ("Judges others by:".to_owned(), "verbal skills".to_owned()),
                                                    ("Influences others by:".to_owned(), "praise, opportunities, favors".to_owned()),
                                                    ("Value to the organization:".to_owned(), "relieves tension; promotes projects and people, including him or herself".to_owned()),
                                                    ("Overuses:".to_owned(), "praise, optimism".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes careless and sentimental; is disorganized".to_owned()),
                                                    ("Fears".to_owned(), "loss of social acceptance and self-worth".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "control of time; objectivity; sense of urgency; emotional control; follow-through on promises and tasks".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Result-Oriented".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "verbalizes ego strength; displays rugged individualism".to_owned()),
                                                    ("Goal:".to_owned(), "dominance and independence".to_owned()),
                                                    ("Judges others by:".to_owned(), "ability to accomplish tasks quickly".to_owned()),
                                                    ("Influences others by:".to_owned(), "force of character; diligence".to_owned()),
                                                    ("Value to the organization:".to_owned(), "persistence; doggedness".to_owned()),
                                                    ("Overuses:".to_owned(), "impatience; \"win-lose\" competition".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes critical and fault-finding; resists participating with a team; may overstep boundaries".to_owned()),
                                                    ("Fears".to_owned(), "others with take advantage of them; slowness, especially in task activities; being a pushover".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "explanation of their reasoning and consideration of other views and ideas about goals and solutions to problems; genuine concern for other; patience and humility".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Specialist".to_owned(),
                                    aspects: vec![("Emotions:".to_owned(), "is calculatingly moderate; accommodates others".to_owned()),
                                                    ("Goal:".to_owned(), "maintenance of the status quo; controlled environment".to_owned()),
                                                    ("Judges others by:".to_owned(), "friendship standards; competence".to_owned()),
                                                    ("Influences others by:".to_owned(), "consistent performance; accommodating others".to_owned()),
                                                    ("Value to the organization:".to_owned(), "plans short term; is predictable, consistent; maintains steady pace".to_owned()),
                                                    ("Overuses:".to_owned(), "modesty; low risk-taking; passive resistance to innovation".to_owned()),
                                                    ("Under pressure:".to_owned(), "becomes adaptable to those in authority and think with the group".to_owned()),
                                                    ("Fears".to_owned(), "change, disorganization".to_owned()),
                                                    ("Would increase effectiveness through:".to_owned(), "public discussion of their ideas; self-confidence based on feedback; shortcut methods".to_owned())],
                                    content: "".to_owned()},
                            Profile {name: "Invalid".to_owned(),
                                    aspects: vec![],
                                    content: "".to_owned()}],

            currentProfile: Profile { name: "".to_owned(), aspects: vec![], content: "".to_owned() }
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { fontSizes, currentPage, currentHighlight, graphType, questions , responses , tally, intensity, profiles, currentProfile} = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 _frame.close();
        //             }
        //         });
        //     });
        // });

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(fontSizes.0, egui::FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(fontSizes.1, egui::FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(fontSizes.2, egui::FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(fontSizes.3, egui::FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(fontSizes.4, egui::FontFamily::Proportional))

        ].into();
        style.visuals.hyperlink_color = style.visuals.text_color();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("DiSC Program");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.selectable_label(*currentPage == Page::Response, "Response Page").clicked() { *currentPage = Page::Response; }
                if ui.selectable_label(*currentPage == Page::Results, "Results Page").clicked() { *currentPage = Page::Results; process(responses, tally, intensity); }
                if ui.selectable_label(*currentPage == Page::Settings, "Settings Page").clicked() { *currentPage = Page::Settings; }
            });

            match *currentPage {
                Page::Response => show_response_instructions(ui),
                _ => ()
            }

            ui.separator();
            ui.add_space(10.0);

            ui.set_min_width(ui.available_width());

            match currentPage {
                Page::Response => show_response_page(currentPage, questions, responses, tally, intensity, ctx, ui),
                Page::Results => show_results_page(currentHighlight, profiles, currentProfile, graphType, tally, intensity, ui),
                Page::Settings => show_settings_page(fontSizes, ui)
            }
        });

        // if false {
        //     egui::Window::new("Window").show(ctx, |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally choose either panels OR windows.");
        //     });
        // }
    }
}

fn show_response_page(currentPage: &mut Page, questions: &mut Vec<Vec<(String, Choice, Choice)>>, responses: &mut Vec<(Choice, Choice)>, tally: &mut (i8, i8, i8, i8), intensity: &mut (i8, i8, i8, i8), ctx: &Context, ui: &mut Ui) {

    egui::Grid::new("Response Page ".to_owned())
        .spacing(Vec2 {x: 10.0, y: 0.0})
        .min_row_height(4.0)
        .show(ui, |ui|{

            ui.add_space(140.0);
            ui.label(RichText::new("MOST").color(Color32::from_rgb(137, 207, 240)));
            ui.label(RichText::new("LEAST").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
        });

        ui.add_space(2.0);

    egui::ScrollArea::vertical().show(ui, |ui| {

        egui::Grid::new("Response Page ".to_owned())
        .striped(true)
        .spacing(Vec2 {x: 10.0, y: 0.0})
        .min_row_height(4.0)
        .show(ui, |ui|{

            for i in 0..questions.len() {
                
                ui.label(RichText::new((i + 1).to_string().to_owned()).strong().color(Color32::from_rgb(137, 207, 240)));

                for j in 0..questions[i].len() {

                    ui.end_row();
                    ui.add_space(10.0);
                    ui.hyperlink_to(questions[i][j].0.to_owned(), "https://www.dictionary.com/browse/".to_owned() + &questions[i][j].0.to_owned());
                    ui.add_space(10.0);
                    ui.radio_value(&mut responses[i].0, questions[i][j].1, "");
                    ui.radio_value(&mut responses[i].1, questions[i][j].2, "");
                    ui.end_row();
                }

                ui.label("");
                ui.end_row();
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
            ui.add_space(50.0);
            if ui.button("Next").clicked() { *currentPage = Page::Results; process(responses, tally, intensity); }
        });

        ui.add_space(20.0);
    });
}

fn show_response_instructions(ui: &mut Ui) {

    egui::CollapsingHeader::new(RichText::new("Instructions").strong().color(Color32::from_rgb(137, 207, 240)))
        .default_open(true)
        .show(ui, |ui| {

            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {

                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("1)").strong().color(Color32::from_rgb(137, 207, 240)));
                    ui.label(RichText::new("Responding").color(Color32::from_rgb(137, 207, 240)));
                });
                ui.add_space(15.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("A.").strong());
                    ui.label(RichText::new("Study the first group of four words below while thinking about yourself in your selected setting or focus."));

                });
                ui.add_space(15.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("B.").strong());
                    ui.add_space(5.0);
                    ui.label(RichText::new("Select"));
                    ui.label(RichText::new("only one word").strong());
                    ui.label(RichText::new("that"));
                    ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
                    ui.label(RichText::new("describes you. Click the"));
                    ui.label(RichText::new("first").strong());
                    ui.label(RichText::new("bubble after the word in the"));
                    ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
                    ui.label(RichText::new("column."));

                });
                ui.add_space(15.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("C.").strong());
                    ui.add_space(5.0);
                    ui.label(RichText::new("Select"));
                    ui.label(RichText::new("only one word").strong());
                    ui.label(RichText::new("that"));
                    ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
                    ui.label(RichText::new("describes you. Click the"));
                    ui.label(RichText::new("second").strong());
                    ui.label(RichText::new("bubble after the word in the"));
                    ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
                    ui.label(RichText::new("column."));

                });
                ui.add_space(15.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("D.").strong());
                    ui.label(RichText::new("Use the same procedure to respond to the remaining groups of descriptive words. Feel free to click on the word to read it's definition if you are unsure what it means."));

                });
                ui.add_space(15.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("REMEMBER:").strong());
                    ui.label(RichText::new("Select only"));
                    ui.label(RichText::new("one").strong());
                    ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
                    ui.label(RichText::new("and"));
                    ui.label(RichText::new("one").strong());
                    ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
                    ui.label(RichText::new("choice for each group."));

                });
                ui.add_space(20.0);

                ui.group(|ui| {
                    
                    ui.label(RichText::new("EXAMPLE 1").strong());
                    ui.add_space(5.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("The individual responding tends to be"));
                        ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
                        ui.label(RichText::new("enthusiastic").italics());
                        ui.label(RichText::new("and"));
                        ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
                        ui.label(RichText::new("satisfied").italics());
                        ui.label(RichText::new("in his or her selected setting."));
                    });
                    ui.add_space(10.0);

                    egui::Grid::new("Example 1".to_owned())
                        .striped(true)
                        .spacing(Vec2 {x: 10.0, y: 0.0})
                        .min_row_height(4.0)
                        .show(ui, |ui|{

                            ui.label("");
                            ui.label(RichText::new("MOST").strong().color(Color32::from_rgb(137, 207, 240)));
                            ui.label(RichText::new("LEAST").strong().color(Color32::from_rgb(137, 207, 240)));
                            ui.end_row();

                            ui.label(RichText::new("1".to_owned()).strong().color(Color32::from_rgb(137, 207, 240)));

                            ui.end_row();
                            ui.add_space(10.0);
                            ui.label("enthusiastic".to_owned());
                            ui.add_space(10.0);
                            ui.radio(true, "");
                            ui.radio(false, "");
                            ui.end_row();

                            ui.end_row();
                            ui.add_space(10.0);
                            ui.label("daring".to_owned());
                            ui.add_space(10.0);
                            ui.radio(false, "");
                            ui.radio(false, "");
                            ui.end_row();

                            ui.end_row();
                            ui.add_space(10.0);
                            ui.label("diplomatic".to_owned());
                            ui.add_space(10.0);
                            ui.radio(false, "");
                            ui.radio(false, "");
                            ui.end_row();

                            ui.end_row();
                            ui.add_space(10.0);
                            ui.label("satisfied".to_owned());
                            ui.add_space(10.0);
                            ui.radio(false, "");
                            ui.radio(true, "");
                            ui.end_row();
                        });
                });
            });

            ui.add_space(10.0);
        });

}

fn show_results_page(currentHighlight: &mut Choice, profiles: &mut Vec<Profile>, currentProfile: &mut Profile, graphType: &mut GraphType, tally: &mut (i8, i8, i8, i8), intensity: &mut (i8, i8, i8, i8), ui: &mut Ui) {

    egui::ScrollArea::vertical().show(ui, |ui| {      

        let x_fmt = |x, _range: &RangeInclusive<f64>| {

            let mut str = "".to_owned();

            if x == 1.0 {
                str = "D".to_owned();
            } else if x == 2.0 {
                str = "i".to_owned();
            } else if x == 3.0 {
                str = "S".to_owned();
            } else if x == 4.0 {
                str = "C".to_owned();
            } else {
                return String::new();
            }
            
            return str.to_owned();
        };

        let y_fmt = |y, _range: &RangeInclusive<f64>| {

            if y >= 1.0 && y <= 28.0 {
                return format!("{}", y);
            } else {
                return String::new();
            }
        };

        let y_spacer = |m: GridInput| {

            return vec![

                GridMark {value: 1.0, step_size: 4.0},
                GridMark {value: 2.0, step_size: 1.0},
                GridMark {value: 3.0, step_size: 1.0},
                GridMark {value: 4.0, step_size: 1.0},

                GridMark {value: 5.0, step_size: 4.0},
                GridMark {value: 6.0, step_size: 1.0},
                GridMark {value: 7.0, step_size: 1.0},
                GridMark {value: 8.0, step_size: 1.0},

                GridMark {value: 9.0, step_size: 4.0},
                GridMark {value: 10.0, step_size: 1.0},
                GridMark {value: 11.0, step_size: 1.0},
                GridMark {value: 12.0, step_size: 1.0},

                GridMark {value: 13.0, step_size: 4.0},
                GridMark {value: 14.0, step_size: 1.0},
                GridMark {value: 15.0, step_size: 1.0},
                GridMark {value: 16.0, step_size: 1.0},

                GridMark {value: 17.0, step_size: 4.0},
                GridMark {value: 18.0, step_size: 1.0},
                GridMark {value: 19.0, step_size: 1.0},
                GridMark {value: 20.0, step_size: 1.0},

                GridMark {value: 21.0, step_size: 4.0},
                GridMark {value: 22.0, step_size: 1.0},
                GridMark {value: 23.0, step_size: 1.0},
                GridMark {value: 24.0, step_size: 1.0},

                GridMark {value: 25.0, step_size: 4.0},
                GridMark {value: 26.0, step_size: 1.0},
                GridMark {value: 27.0, step_size: 1.0},
                GridMark {value: 28.0, step_size: 1.0},
            ]
        };

        ui.set_min_width(ui.available_width());
        egui::Grid::new("Numbers")
            .num_columns(5)
            .striped(true)
            .min_row_height(30.0)
            .min_col_width(ui.available_width() / 5.0)
            .show(ui, |ui| {

                ui.label("");
                if ui.selectable_label(*currentHighlight == Choice::A, RichText::new("D").strong().color(Color32::from_rgb(137, 207, 240))).clicked() { if *currentHighlight == Choice::A { *currentHighlight = Choice::NONE; } else { *currentHighlight = Choice::A; }};
                if ui.selectable_label(*currentHighlight == Choice::B, RichText::new("i").strong().color(Color32::from_rgb(137, 207, 240))).clicked() { if *currentHighlight == Choice::B { *currentHighlight = Choice::NONE; } else { *currentHighlight = Choice::B; }};
                if ui.selectable_label(*currentHighlight == Choice::C, RichText::new("S").strong().color(Color32::from_rgb(137, 207, 240))).clicked() { if *currentHighlight == Choice::C { *currentHighlight = Choice::NONE; } else { *currentHighlight = Choice::C; }};
                if ui.selectable_label(*currentHighlight == Choice::D, RichText::new("C").strong().color(Color32::from_rgb(137, 207, 240))).clicked() { if *currentHighlight == Choice::D { *currentHighlight = Choice::NONE; } else { *currentHighlight = Choice::D; }};

                // ui.end_row();

                // ui.label(RichText::new("Tally"));
                // ui.label(&tally.0.to_string());
                // ui.label(&tally.1.to_string());
                // ui.label(&tally.2.to_string());
                // ui.label(&tally.3.to_string());

                ui.end_row();

                ui.label(RichText::new("Intensity"));
                ui.label(&intensity.0.to_string());
                ui.label(&intensity.1.to_string());
                ui.label(&intensity.2.to_string());
                ui.label(&intensity.3.to_string());

                ui.end_row();

                ui.label(RichText::new("Segment"));
                ui.label(&intensity_to_segment(intensity.0).to_string());
                ui.label(&intensity_to_segment(intensity.1).to_string());
                ui.label(&intensity_to_segment(intensity.2).to_string());
                ui.label(&intensity_to_segment(intensity.3).to_string());

                ui.end_row();
            });

        ui.add_space(25.0);
        ui.set_min_width(ui.available_width() - 100.0);

        match currentHighlight {

            Choice::A => show_d_highlights(ui),
            Choice::B => show_i_highlights(ui),
            Choice::C => show_s_highlights(ui),
            Choice::D => show_c_highlights(ui),
            _ => ()

        }

        ui.separator();

        ui.collapsing(RichText::new("Graph").strong().color(Color32::from_rgb(137, 207, 240)), |ui| {

            ui.add_space(10.0);

            ui.horizontal_wrapped(|ui| {

                if ui.selectable_label(*graphType == GraphType::Line, "Line").clicked() { *graphType = GraphType::Line; }
                if ui.selectable_label(*graphType == GraphType::Bar, "Bar").clicked() { *graphType = GraphType::Bar; }

            });
            ui.add_space(20.0);

            Plot::new("Graph")
                .data_aspect(6.5 / 28.0)
                .view_aspect(0.75)
                .show_x(false)
                .show_y(false)
                .height(500.0)
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .show_background(false)
                .x_axis_formatter(x_fmt)
                .y_axis_formatter(y_fmt)
                .y_grid_spacer(y_spacer)
                .show(ui, |plot_ui| {

                    let series: PlotPoints = PlotPoints::Owned(vec![PlotPoint::new(1, intensity.0), PlotPoint::new(2, intensity.1), PlotPoint::new(3, intensity.2), PlotPoint::new(4, intensity.3)]);
                    let line: egui::plot::Line = Line::new(series);      

                    let dBar: egui::plot::Bar = Bar::new(1.0, intensity.0 as f64);
                    let iBar: egui::plot::Bar = Bar::new(2.0, intensity.1 as f64);
                    let sBar: egui::plot::Bar = Bar::new(3.0, intensity.2 as f64);
                    let cBar: egui::plot::Bar = Bar::new(4.0, intensity.3 as f64);

                    let barChart: egui::plot::BarChart = BarChart::new(vec![dBar, iBar, sBar, cBar]);   

                    match graphType {
                        GraphType::Line => plot_ui.line(line),
                        GraphType::Bar => plot_ui.bar_chart(barChart)
                    }

                    plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [4.5, 28.0]));
                });
        });

        ui.add_space(10.0);
        ui.separator();

        ui.collapsing(RichText::new("Profile Pattern").strong().color(Color32::from_rgb(137, 207, 240)), |ui| {

            let seg: (i8, i8, i8, i8) = (intensity_to_segment(intensity.0), intensity_to_segment(intensity.1), intensity_to_segment(intensity.2), intensity_to_segment(intensity.3));

            *currentProfile = profilePatternLookup(profiles, seg);
            show_profile_section(currentProfile, ui);
        });
    });
}

fn show_profile_section(p: &mut Profile, ui: &mut Ui) {

    ui.group(|ui| {

        ui.add_space(5.0);
        ui.label(RichText::new(p.name.to_owned() + " Pattern").strong());
        ui.add_space(15.0);

        ui.horizontal_wrapped(|ui| {

            for pair in &p.aspects {

                ui.small(RichText::new(pair.0.to_owned()).strong().color(Color32::from_rgb(137, 207, 240)));
                ui.small(RichText::new(pair.1.to_owned()));
                ui.end_row();
            }
        });

        ui.add_space(20.0);

        ui.horizontal_wrapped(|ui| {

            ui.small(RichText::new(p.content.to_owned()));
            ui.end_row();
        });
    });
}

fn show_settings_page(fontSizes: &mut (f32, f32, f32, f32, f32), ui: &mut Ui) {

    ui.add(egui::Slider::new(&mut fontSizes.0, 8.0..=32.0).text("Heading"));
    ui.add(egui::Slider::new(&mut fontSizes.1, 8.0..=32.0).text("Body"));
    //ui.add(egui::Slider::new(&mut fontSizes.2, 8.0..=32.0).text("Monospace"));
    ui.add(egui::Slider::new(&mut fontSizes.3, 8.0..=32.0).text("Button"));
    ui.add(egui::Slider::new(&mut fontSizes.4, 8.0..=32.0).text("Small"));
}

// Helper Functions

fn process(responses: &mut Vec<(Choice, Choice)>, tally: &mut (i8, i8, i8, i8), intensity: &mut (i8, i8, i8, i8)) {

    tally.0 = 0;
    tally.1 = 0;
    tally.2 = 0;
    tally.3 = 0;

    for response in responses {

        match response.0 {

            Choice::A => tally.0 += 1,
            Choice::B => tally.1 += 1,
            Choice::C => tally.2 += 1,
            Choice::D => tally.3 += 1,
            _ => ()
        }

        match response.1 {

            Choice::A => tally.0 -= 1,
            Choice::B => tally.1 -= 1,
            Choice::C => tally.2 -= 1,
            Choice::D => tally.3 -= 1,
            _ => ()
        }
    }

    intensity.0 = (27.38232853 / (1.0 + 0.297148753 * E.powf(-0.1801194362 * tally.0 as f64))).clamp(1.0, 28.0) as i8; // Logistic Regression
    intensity.1 = (28.13823356 / (1.0 + 1.242064677 * E.powf(-0.2464025952 * tally.1 as f64))).clamp(1.0, 28.0) as i8; // Logistic Regression
    intensity.2 = (29.51533099 / (1.0 + 2.209999802 * E.powf(-0.1941614665  * tally.2 as f64))).clamp(1.0, 28.0) as i8; // Logistic Regression
    intensity.3 = (27.31404101 / (1.0 + 0.5608447664 * E.powf(-0.2479183241  * tally.3 as f64))).clamp(1.0, 28.0) as i8; // Logistic Regression
}

fn intensity_to_segment(val: i8) -> i8 {

    return ((val - 1) / 4) + 1;
}

fn profilePatternLookup(profiles: &mut Vec<Profile>, seg: (i8, i8, i8, i8)) -> Profile {

    let d = seg.0;
    let i = seg.1;
    let s = seg.2;
    let c = seg.3;

    let mut num = 15;

    if r(d, 5, 7) && r(i, 1, 4) && r(s, 5, 7) && r(c, 1, 4) { num = 0; }
    else if r(d, 1, 5) && r(i, 5, 7)  && r(s, 5, 7) && r(c, 1, 4) && i < s { num = 1; }
    else if r(d, 1, 7) && r(i, 5, 7) && r(s, 1, 4) && r(c, 5, 7) { num = 2; }
    else if r(d, 1, 5) && r(i, 5, 7) && r(s, 5, 7) && r(c, 1, 4) && i >= s { num = 3; }
    else if r(d, 5, 7) && r(i, 1, 4) && r(s, 1, 4) && r(c, 5, 7) { num = 4; }
    else if r(d, 5, 7) && r(i, 1, 3) && r(s, 1, 4) && r(c, 1, 4) { num = 5; }
    else if r(d, 5, 7) && r(i, 5, 7) && r(s, 1, 7) && r(c, 1, 4) { num = 6; }
    else if r(d, 5, 7) && r(i, 1, 4) && r(s, 5, 7) && r(c, 5, 7) { num = 7; }
    else if r(d, 1, 4) && r(i, 1, 4) && r(s, 1, 4) && r(c, 5, 7) { num = 8; }
    else if r(d, 1, 4) && r(i, 1, 4) && r(s, 5, 7) && r(c, 5, 7) { num = 9; }
    else if r(d, 5, 6) && r(i, 6, 7) && r(s, 1, 5) && r(c, 1, 4) && d < i { num = 10; }
    else if r(d, 1, 4) && r(i, 5, 7) && r(s, 1, 7) && r(c, 5, 7) { num = 11; }
    else if r(d, 1, 4) && r(i, 5, 7) && r(s, 1, 4) && r(c, 1, 4) { num = 12; }
    else if r(d, 5, 7) && r(i, 4, 6) && r(s, 1, 5) && r(c, 1, 4) { num = 13; }
    else if r(d, 1, 4) && r(i, 1, 4) && r(s, 5, 7) && r(c, 1, 4) { num = 14; }

    return profiles[num].clone();
}

fn r(val: i8, low: i8, high: i8) -> bool {

    return val >= low && val <= high;
}

fn show_d_highlights(ui: &mut Ui) {

    ui.horizontal_wrapped(|ui| {

        ui.small(RichText::new("DOMINANCE").strong());
        ui.end_row();
        ui.small("Emphasis is on shaping the environment by overcoming opposition to accomplish results.");
        ui.end_row();
    });

    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    ui.columns(4, |col| {

        col[0].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person's tendencies include").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• getting immediate results");
            ui.end_row();
            ui.small("• causing action");
            ui.end_row();
            ui.small("• accepting challanges");
            ui.end_row();
            ui.small("• making quick decisions");
            ui.end_row();
            ui.small("• questioning the status quo");
            ui.end_row();
            ui.small("• taking authority");
            ui.end_row();
            ui.small("• managing trouble");
            ui.end_row();
            ui.small("• solving problems");
            ui.end_row();
        });
        
        col[1].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person desires an environment that includes").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• power and authority");
            ui.end_row();
            ui.small("• prestige and challenge");
            ui.end_row();
            ui.small("• opportunities for individual accomplishments");
            ui.end_row();
            ui.small("• a wipe scrope of operations");
            ui.end_row();
            ui.small("• direct answers");
            ui.end_row();
            ui.small("• opportunities for advancement");
            ui.end_row();
            ui.small("• freedom from controls and supervision");
            ui.end_row();
            ui.small("• many new and varied activities");
            ui.end_row();
        });

        col[2].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person needs others who").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• weigh pros and cons");
            ui.end_row();
            ui.small("• calculate risks");
            ui.end_row();
            ui.small("• use caution");
            ui.end_row();
            ui.small("• create a predictable environment");
            ui.end_row();
            ui.small("• research facts");
            ui.end_row();
            ui.small("• deliberate before deciding");
            ui.end_row();
            ui.small("• recognize the needs of others");
            ui.end_row();
        });

        col[3].horizontal_wrapped(|ui| {

            ui.small(RichText::new("To be more effective, this person needs").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• to receive difficult assignments");
            ui.end_row();
            ui.small("• to understand that they need people");
            ui.end_row();
            ui.small("• to base techniques on practical experience");
            ui.end_row();
            ui.small("• to receive an occasional shock");
            ui.end_row();
            ui.small("• to identify with a group");
            ui.end_row();
            ui.small("• to verbalize reasons for conclusions");
            ui.end_row();
            ui.small("• to be aware of existing sanctions");
            ui.end_row();
            ui.small("• to pace self and to relax more");
            ui.end_row();
        });
    });

    ui.add_space(25.0);
}

fn show_i_highlights(ui: &mut Ui) {

    ui.horizontal_wrapped(|ui| {

        ui.small(RichText::new("INFLUENCE").strong());
        ui.end_row();
        ui.small("Emphasis is on shaping the environment by influencing or persuading others.");
        ui.end_row();
    });

    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    ui.columns(4, |col| {

        col[0].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person's tendencies include").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• contacting people");
            ui.end_row();
            ui.small("• making a favorable impression");
            ui.end_row();
            ui.small("• being articulate");
            ui.end_row();
            ui.small("• creating a motivating environment");
            ui.end_row();
            ui.small("• generating enthusiasm");
            ui.end_row();
            ui.small("• entertaining people");
            ui.end_row();
            ui.small("• viewing people and situations with optimism");
            ui.end_row();
            ui.small("• participating in a group");
            ui.end_row();
        });
        
        col[1].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person desires an environment that includes").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• popularity, social recognition");
            ui.end_row();
            ui.small("• public recognition of ability");
            ui.end_row();
            ui.small("• freedom of expression");
            ui.end_row();
            ui.small("• group activities outside of job");
            ui.end_row();
            ui.small("• democratic relationships");
            ui.end_row();
            ui.small("• freedom from control and detail");
            ui.end_row();
            ui.small("• opportunities to verbalize proposals");
            ui.end_row();
            ui.small("• coaching and counseling");
            ui.end_row();
            ui.small("• favorable working conditions");
            ui.end_row();
        });

        col[2].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person needs others who").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• concentrate on the task");
            ui.end_row();
            ui.small("• seek facts");
            ui.end_row();
            ui.small("• speak directly");
            ui.end_row();
            ui.small("• respect sincerity");
            ui.end_row();
            ui.small("• develop systematic approaches");
            ui.end_row();
            ui.small("• prefer to deal with things instead of people");
            ui.end_row();
            ui.small("• take a logical approach");
            ui.end_row();
            ui.small("• demonstrate individual follow-through");
            ui.end_row();
        });

        col[3].horizontal_wrapped(|ui| {

            ui.small(RichText::new("To be more effective, this person needs").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• to control time, if 'D' or 'S' is low");
            ui.end_row();
            ui.small("• to make objective decisions");
            ui.end_row();
            ui.small("• to use hands-on management");
            ui.end_row();
            ui.small("• to be more realistic when appraising others");
            ui.end_row();
            ui.small("• to make priorities and deadlines");
            ui.end_row();
            ui.small("• to be more firm with others, if D is low");
            ui.end_row();
        });
    });

    ui.add_space(25.0);
}

fn show_s_highlights(ui: &mut Ui) {

    ui.horizontal_wrapped(|ui| {

        ui.small(RichText::new("STEADINESS").strong());
        ui.end_row();
        ui.small("Emphasis is on cooperating with others within existing circumstances to carry out the task.");
        ui.end_row();
    });

    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    ui.columns(4, |col| {

        col[0].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person's tendencies include").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• performing in a consistent, predictable manner");
            ui.end_row();
            ui.small("• demonstrating patience");
            ui.end_row();
            ui.small("• developing specialized skills");
            ui.end_row();
            ui.small("• helping others");
            ui.end_row();
            ui.small("• showing loyalty");
            ui.end_row();
            ui.small("• being a good listener");
            ui.end_row();
            ui.small("• calming excited people");
            ui.end_row();
            ui.small("• creating a stable harmonious work environment");
            ui.end_row();
        });
        
        col[1].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person desires an environment that includes").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• maintenance of the status quo unless given reasons for change");
            ui.end_row();
            ui.small("• predictable routines");
            ui.end_row();
            ui.small("• credit for work accomplished");
            ui.end_row();
            ui.small("• minimal work infringement on home life");
            ui.end_row();
            ui.small("• sincere appreciation");
            ui.end_row();
            ui.small("• identification with a group");
            ui.end_row();
            ui.small("• standard operating procedures");
            ui.end_row();
            ui.small("• minimal conflicts");
            ui.end_row();
        });

        col[2].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person needs others who").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• react quickly to unexpected change");
            ui.end_row();
            ui.small("• stretch toward the challenges of accepted tasks");
            ui.end_row();
            ui.small("• become involved in more than one thing");
            ui.end_row();
            ui.small("• are self-promoting");
            ui.end_row();
            ui.small("• apply pressure on others");
            ui.end_row();
            ui.small("• work comfortably in an unpredictable environment");
            ui.end_row();
            ui.small("• help to prioritize work");
            ui.end_row();
            ui.small("• are flexible in work procedures");
            ui.end_row();
        });

        col[3].horizontal_wrapped(|ui| {

            ui.small(RichText::new("To be more effective, this person needs").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• to be conditioned prior to change");
            ui.end_row();
            ui.small("• to validate self-worth");
            ui.end_row();
            ui.small("• to know how personal effort contributes to the group effort");
            ui.end_row();
            ui.small("• to have colleagues of similar competence and sincerity");
            ui.end_row();
            ui.small("• to know task guidelines");
            ui.end_row();
            ui.small("• to have creativity encouraged");
            ui.end_row();
        });
    });

    ui.add_space(25.0);
}

fn show_c_highlights(ui: &mut Ui) {

    ui.horizontal_wrapped(|ui| {

        ui.small(RichText::new("CONSCIENTIOUSNESS").strong());
        ui.end_row();
        ui.small("Emphasis is on working conscientiously within existing circumstances to ensure quality and accuracy.");
        ui.end_row();
    });

    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    ui.columns(4, |col| {

        col[0].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person's tendencies include").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• adhering to key directives and standards");
            ui.end_row();
            ui.small("• concentratingon key details");
            ui.end_row();
            ui.small("• thinking analytically, weighing pros and cons");
            ui.end_row();
            ui.small("• being diplomatic with people");
            ui.end_row();
            ui.small("• using subtle or indirect approaches to conflict");
            ui.end_row();
            ui.small("• checking for accuracy");
            ui.end_row();
            ui.small("• analyzing performance critically");
            ui.end_row();
            ui.small("• using a systematic approach to situations or activities");
            ui.end_row();
        });
        
        col[1].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person desires an environment that includes").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• clearly defined performance expectations");
            ui.end_row();
            ui.small("• values of quality and accuracy");
            ui.end_row();
            ui.small("• a reserved, business-like atmosphere");
            ui.end_row();
            ui.small("• opportunities to demonstrate expertise");
            ui.end_row();
            ui.small("• control over factors that affect their performance");
            ui.end_row();
            ui.small("• opportunities to ask \"why\" questions");
            ui.end_row();
            ui.small("• recognition for specific skills and accomplishments");
            ui.end_row();
        });

        col[2].horizontal_wrapped(|ui| {

            ui.small(RichText::new("This person needs others who").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• delegate important tasks");
            ui.end_row();
            ui.small("• make quick decisions");
            ui.end_row();
            ui.small("• use policies only as guidelines");
            ui.end_row();
            ui.small("• compromise with the opposition");
            ui.end_row();
            ui.small("• state unpopular positions");
            ui.end_row();
            ui.small("• initiate and facilitate discussions");
            ui.end_row();
            ui.small("• encourage teamwork");
            ui.end_row();
        });

        col[3].horizontal_wrapped(|ui| {

            ui.small(RichText::new("To be more effective, this person needs").color(Color32::from_rgb(137, 207, 240)));
            ui.end_row();
            ui.small("• to have time to plan carefully");
            ui.end_row();
            ui.small("• to know exact job descriptions and performance objectives");
            ui.end_row();
            ui.small("• to schedule performance appraisals");
            ui.end_row();
            ui.small("• to receieve specific feedback on performance");
            ui.end_row();
            ui.small("• to respect people's personal worth as much as their accomplishments");
            ui.end_row();
            ui.small("• to develop tolerance for conflict");
            ui.end_row();
        });
    });

    ui.add_space(25.0);
}