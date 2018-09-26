pub fn check(candidate: &str) -> bool {
  let mut candidate_without_separators = candidate.to_string();
  // removing separators
  candidate_without_separators.retain(|c| c != '-' && c != ' ');
  // converting to lowercase
  candidate_without_separators = candidate_without_separators.to_lowercase();
  unsafe {
    // convert to vector
    let mut candidate_vec: Vec<u8> = candidate_without_separators.as_mut_vec().to_vec();
    // sort vector for dedup to work
    candidate_vec.sort();
    // clone to compare
    let candidate_vec_dup = candidate_vec.clone();
    // remove all duplicates
    candidate_vec.dedup();
    // compare string without duplicates and with duplicates
    if candidate_vec == candidate_vec_dup {
      return true;
    }
  }
  false
}
