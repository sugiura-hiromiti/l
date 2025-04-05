//!My algorithm collection

///This `fn` uses **manacher's algorithm**
pub fn longest_palindrome(s: String) -> String {
	let s_with_bogus = format!("|{}", s.chars().map(|c| c.to_string() + "|",).collect::<String>());
	let (mut center, mut radius, len) = (0, 0, s_with_bogus.len());
	let mut palind_radii = vec![0; len];

	while center < len {
		while center - radius > 0
			&& center + radius < len - 1
			&& s_with_bogus[center - radius - 1..center - radius]
				== s_with_bogus[center + radius + 1..center + radius + 2]
		{
			radius += 1;
		}

		palind_radii[center] = radius;

		let (old_center, old_radius) = (center, radius);
		center += 1;
		radius = 0;
		while center <= old_center + old_radius {
			let mirrored_center = old_center - (center - old_center);
			let max_mirrored_radi = old_center + old_radius - center;

			if palind_radii[mirrored_center] < max_mirrored_radi {
				palind_radii[center] = palind_radii[mirrored_center];
				center += 1;
			} else if palind_radii[mirrored_center] > max_mirrored_radi {
				// NOTE: this is legal because max_mirrored_radi+1 has already proved ilegal
				palind_radii[center] = max_mirrored_radi;

				center += 1;
			} else {
				radius = max_mirrored_radi; //palind_radii[mirrored_center] = max_mirrored_radi
				break;
			}
		}
	}

	radius = 0;
	for (i, &r) in palind_radii.iter().enumerate() {
		if radius < r {
			radius = r;
			center = i;
		}
	}

	s_with_bogus[center - radius..center + radius + 1].chars().filter(|&c| c != '|').collect()
}

///See more detail at [here](https://leetcode.com/problems/regular-expression-matching/description/)
///
///This `fn` supports regular-expression-matching with . & * where:
/// - '.' matches any single character
/// - '*' matches zero or more of the preceding element
///
/// # Example
///
/// ```rust
/// use l::l::utl::algorithm::regex_match;
/// assert_eq!(regex_match("bbbba".to_string(), ".*a*a".to_string()), true);
/// assert_eq!(regex_match("a".to_string(), ".*..a*".to_string()), false);
/// assert_eq!(regex_match("ab".to_string(), ".*..".to_string()), true);
/// ```
pub fn regex_match(s: String, p: String) -> bool {
	if p.is_empty() {
		return s.is_empty();
	}

	let first_match = !s.is_empty() && (p.chars().next() == s.chars().next() || p.starts_with('.'));

	if p.len() >= 2 && p.chars().nth(1) == Some('*') {
		regex_match(s.clone(), p[2..].to_string())
			|| (first_match && regex_match(s[1..].to_string(), p))
	} else {
		first_match && regex_match(s[1..].to_string(), p[1..].to_string())
	}
}
