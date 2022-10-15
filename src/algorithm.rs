//!My algorithm collection

///This `fn` uses **manacher's algorithm**
pub fn longest_palindrome(s: String,) -> String {
   let s_with_bogus = format!("|{}", s.chars().map(|c| c.to_string() + "|",).collect::<String>());
   let (mut center, mut radius, len,) = (0, 0, s_with_bogus.len(),);
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

      let (old_center, old_radius,) = (center, radius,);
      center += 1;
      radius = 0;
      while center <= old_center + old_radius {
         let mirrored_center = old_center - (center - old_center);
         let max_mirrored_radi = old_center + old_radius - center;

         if palind_radii[mirrored_center] < max_mirrored_radi {
            palind_radii[center] = palind_radii[mirrored_center];
            center += 1;
         } else if palind_radii[mirrored_center] > max_mirrored_radi {
            palind_radii[center] = max_mirrored_radi; //NOTE this is legal because
                                                      //max_mirrored_radi+1 has already proved
                                                      //ilegal
            center += 1;
         } else {
            radius = max_mirrored_radi; //palind_radii[mirrored_center] = max_mirrored_radi
            break;
         }
      }
   }

   radius = 0;
   for (i, &r,) in palind_radii.iter().enumerate() {
      if radius < r {
         radius = r;
         center = i;
      }
   }

   s_with_bogus[center - radius..center + radius + 1].chars().filter(|&c| c != '|',).collect()
}
