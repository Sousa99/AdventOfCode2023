
// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
pub type ValueReading = i64;
pub type ValueVariation = Vec<ValueReading>;

// =============================================== AUXILIARY FUNCTIONS ===============================================
pub fn compute_variations_steps(variation: ValueVariation) -> Vec<ValueVariation> {
  
  let mut finished_computing = false;
  let mut steps_variations: Vec<ValueVariation> = vec![variation];
  while !finished_computing {

    // Compute next step
    let last_variation = steps_variations.last().unwrap();
    let new_step: ValueVariation = last_variation.iter().zip(last_variation.iter().skip(1))
      .map(|(&prev, &next)| next - prev)
      .collect();

    // Check if it should stop iteration
    finished_computing = new_step.iter().all(|&value| value == 0);

    // Update general tracker
    steps_variations.push(new_step);
  }

  steps_variations
}

pub fn estimate_next_value(variation_steps: &Vec<ValueVariation>) -> ValueReading {
  variation_steps.into_iter()
    .map(|step| step.last().unwrap().to_owned())
    .sum()
}

pub fn estimate_prev_value(variation_steps: &Vec<ValueVariation>) -> ValueReading {
  fn compute_signal(index: usize) -> i64 {
    match index % 2 {
      0 => 1,
      1 => -1,
      _ => panic!("🚨 Impossible for it to happen!")
    }
  }

  variation_steps.into_iter().enumerate()
    .map(|(index, step)| (compute_signal(index), step.first().unwrap().to_owned()))
    .map(|(signal, step)| signal * step)
    .sum()
}

// ================================================= IMPLEMENTATIONS =================================================
