use anyhow::anyhow;
use enum_all_variants::AllVariants;

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq, AllVariants)]
pub enum Landmark {
	S,
	N,
	Ba,
	Pg,
	A,
	Gn,
	B,
	ANS,
	CoR,
	CoL,
	IncR,
	IncL,
	GoL,
	GoR,
	LOR,
	LOL,
	InS,
	InInf,
	InSInInf,
	MolSupR,
	MolSupL,
	MolInfR,
	MolInfL,
	OrR,
	OrL,
}

impl std::str::FromStr for Landmark {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s_lower = s.to_lowercase();
		match s_lower.as_str() {
			"s" | "sella" => Ok(Self::S),
			"n" | "nasion" => Ok(Self::N),
			"ba" | "basion" => Ok(Self::Ba),
			"pg" | "pogonion" => Ok(Self::Pg),
			"a" | "a point" | "point a" => Ok(Self::A),
			"gn" | "gnathion" => Ok(Self::Gn),
			"b" | "b point" | "point b" => Ok(Self::B),
			"ans" | "anterior nasal spine" => Ok(Self::ANS),
			"cor" | "condylion, r" => Ok(Self::CoR),
			"col" | "condylion, l" => Ok(Self::CoL),
			"incr" | "incisura, r" => Ok(Self::IncR),
			"incl" | "incisura, l" => Ok(Self::IncL),
			"gol" | "gonion, l" => Ok(Self::GoL),
			"gor" | "gonion, r" => Ok(Self::GoR),
			"lo_r" | "latero-orbital point, r" => Ok(Self::LOR),
			"lo_l" | "latero-orbital point, l" => Ok(Self::LOL),
			"ins" | "midpoint upper incisors" => Ok(Self::InS),
			"ininf" | "midpoint lower incisors" => Ok(Self::InInf),
			"ins-ininf" | "midpoint between incisors" => Ok(Self::InSInInf),
			"molsupr" | "cusp upper molar, r" => Ok(Self::MolSupR),
			"molsupl" | "cusp upper molar, l" => Ok(Self::MolSupL),
			"molinfr" | "cusp lower molar, r" => Ok(Self::MolInfR),
			"molinfl" | "cusp lower molar, l" => Ok(Self::MolInfL),
			"orr" | "orbitale, r" => Ok(Self::OrR),
			"orl" | "orbitale, l" => Ok(Self::OrL),
			_ => Err(anyhow!("Failed to parse landmark: '{}'", s)),
		}
	}
}

impl std::fmt::Display for Landmark {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match &self {
				Self::S => "S",
				Self::N => "N",
				Self::Ba => "Ba",
				Self::Pg => "Pg",
				Self::A => "A",
				Self::Gn => "Gn",
				Self::B => "B",
				Self::ANS => "ANS",
				Self::CoR => "CoR",
				Self::CoL => "CoL",
				Self::IncR => "IncR",
				Self::IncL => "IncL",
				Self::GoL => "GoL",
				Self::GoR => "GoR",
				Self::LOR => "LO_R",
				Self::LOL => "LO_L",
				Self::InS => "InS",
				Self::InInf => "InInf",
				Self::InSInInf => "InS-InInf",
				Self::MolSupR => "MolSupR",
				Self::MolSupL => "MolSupL",
				Self::MolInfR => "MolInfR",
				Self::MolInfL => "MolInfL",
				Self::OrR => "OrR",
				Self::OrL => "OrL",
			}
		)
	}
}
