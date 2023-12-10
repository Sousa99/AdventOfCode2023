// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
pub type EntityID = u64;

struct EntityMapEntry {
  destination_start: EntityID,
  source_start: EntityID,
  range_size: u64,
}

struct EntityMap {
  maps: Vec<EntityMapEntry>
}

pub struct Almanac {
  seeds: Vec<EntityID>,
  seed_to_soil: EntityMap,
  soil_to_fertilizer: EntityMap,
  fertilizer_to_water: EntityMap,
  water_to_light: EntityMap,
  light_to_temperature: EntityMap,
  temperature_to_humidity: EntityMap,
  humidity_to_location: EntityMap,
}

#[derive(Clone)]
pub struct RangeItem {
  pub range_start: EntityID,
  pub range_end: EntityID,
}

type Range = Vec<RangeItem>;

pub struct AlmanacRange {
  seeds: Range,
  seed_to_soil: EntityMap,
  soil_to_fertilizer: EntityMap,
  fertilizer_to_water: EntityMap,
  water_to_light: EntityMap,
  light_to_temperature: EntityMap,
  temperature_to_humidity: EntityMap,
  humidity_to_location: EntityMap,
}

// =============================================== AUXILIARY FUNCTIONS ===============================================
fn parse_entity_map_entry(line: &str) -> EntityMapEntry {
  let mut ids: Vec<EntityID> = line.split_whitespace()
    .into_iter()
    .map(|id| id.parse().unwrap())
    .collect();

  EntityMapEntry {
    destination_start: ids.remove(0),
    source_start: ids.remove(0),
    range_size: ids.remove(0)
  }
}

fn parse_entity_map(mut lines: Vec<&str>) -> EntityMap {
  let _ = lines.remove(0);
  let maps = lines.into_iter()
    .map(|line| parse_entity_map_entry(line))
    .collect();

  EntityMap { maps }
}

pub fn parse_almanac(lines: &Vec<String>) -> Almanac {
  let mut almanac_groups: Vec<Vec<&str>> = lines.split(|line| line.is_empty())
    .into_iter()
    .map(|group| group.into_iter()
      .map(|line| line.as_ref())
      .collect())
    .collect();

  let seeds: Vec<EntityID> = almanac_groups.remove(0)
    .remove(0)
    .strip_prefix("seeds: ").unwrap()
    .split_whitespace()
    .map(|seed_number| seed_number.parse().unwrap())
    .collect();

  let seed_to_soil = parse_entity_map(almanac_groups.remove(0));
  let soil_to_fertilizer = parse_entity_map(almanac_groups.remove(0));
  let fertilizer_to_water = parse_entity_map(almanac_groups.remove(0));
  let water_to_light = parse_entity_map(almanac_groups.remove(0));
  let light_to_temperature = parse_entity_map(almanac_groups.remove(0));
  let temperature_to_humidity = parse_entity_map(almanac_groups.remove(0));
  let humidity_to_location = parse_entity_map(almanac_groups.remove(0));

  Almanac {
    seeds,
    seed_to_soil, soil_to_fertilizer,
    fertilizer_to_water, water_to_light,
    light_to_temperature, temperature_to_humidity, humidity_to_location }
}

pub fn parse_almanac_range(lines: &Vec<String>) -> AlmanacRange {
  let mut almanac_groups: Vec<Vec<&str>> = lines.split(|line| line.is_empty())
    .into_iter()
    .map(|group| group.into_iter()
      .map(|line| line.as_ref())
      .collect())
    .collect();

  let seed_ids: Vec<EntityID> = almanac_groups.remove(0)
    .remove(0)
    .strip_prefix("seeds: ").unwrap()
    .split_whitespace()
    .map(|seed_number| seed_number.parse().unwrap())
    .collect();
  let seeds: Range = seed_ids.chunks(2)
    .into_iter()
    .map(|chunk| {
      let range_start = *chunk.get(0).unwrap();
      let range_end = range_start + *chunk.get(1).unwrap() - 1;

      RangeItem { range_start, range_end }})
    .collect();

  let seed_to_soil = parse_entity_map(almanac_groups.remove(0));
  let soil_to_fertilizer = parse_entity_map(almanac_groups.remove(0));
  let fertilizer_to_water = parse_entity_map(almanac_groups.remove(0));
  let water_to_light = parse_entity_map(almanac_groups.remove(0));
  let light_to_temperature = parse_entity_map(almanac_groups.remove(0));
  let temperature_to_humidity = parse_entity_map(almanac_groups.remove(0));
  let humidity_to_location = parse_entity_map(almanac_groups.remove(0));

  AlmanacRange {
    seeds,
    seed_to_soil, soil_to_fertilizer,
    fertilizer_to_water, water_to_light,
    light_to_temperature, temperature_to_humidity, humidity_to_location }
}

// ================================================= IMPLEMENTATIONS =================================================
impl EntityMapEntry {

  fn convert_source_to_dest(&self, source_id: EntityID) -> EntityID {
    source_id - self.source_start + self.destination_start
  }

  fn match_source(&self, source_id: EntityID) -> Option<EntityID> {
    if source_id >= self.source_start && source_id < (self.source_start + self.range_size) {
      return Some(self.convert_source_to_dest(source_id));
    }

    None
  }

  fn match_range(&self, range: RangeItem) -> (Option<RangeItem>, Option<RangeItem>, Option<RangeItem>) {
    let mut before_range: Option<RangeItem> = None;
    let mut between_range: Option<RangeItem> = None;
    let mut after_range: Option<RangeItem> = None;

    // Check if there is a range out of scope before
    if range.range_start < self.source_start {
      before_range = Some(RangeItem {
        range_start: range.range_start,
        range_end: EntityID::min(self.source_start - 1, range.range_end) });
    }

    // Check if there is a range in between the map item
    let range_outside = (range.range_end < self.source_start) || (range.range_start >= (self.source_start + self.range_size));
    if !range_outside {
      let between_source_start = EntityID::max(self.source_start, range.range_start);
      let between_source_end = EntityID::min(self.source_start + self.range_size - 1, range.range_end);

      between_range = Some(RangeItem {
        range_start: self.convert_source_to_dest(between_source_start),
        range_end: self.convert_source_to_dest(between_source_end) });
    }

    // Check if there is a range out of scope after
    if range.range_end >= (self.source_start + self.range_size) {
      after_range = Some(RangeItem {
        range_start: EntityID::max(self.source_start + self.range_size, range.range_start),
        range_end: range.range_end });
    }

    (before_range, between_range, after_range)
  }
}

impl EntityMap {

  fn match_source(&self, source_id: EntityID) -> EntityID {
    self.maps.iter()
      .map(|entity_map| entity_map.match_source(source_id))
      .find_map(|destination_result| destination_result)
      .unwrap_or(source_id)
  }

  fn match_range(&self, range: Range) -> Range {
    let mut parsed_range: Range = Vec::new();

    let unparsed_range = self.maps.iter()
      .fold(range, |unparsed_range, map| {
        unparsed_range.into_iter()
          .flat_map(|unparsed_range_item| {
            let (before, between, after) = map.match_range(unparsed_range_item);

            between.map(|between| parsed_range.push(between));

            vec![before, after].into_iter()
              .filter_map(|range_item| range_item)
          })
          .collect::<Vec<RangeItem>>()
      });

    parsed_range.extend(unparsed_range);
    parsed_range
  }
}

impl Almanac {

  pub fn convert_seeds_to_locations(&self) -> Vec<EntityID> {

    fn convert_ids(ids: &Vec<EntityID>, map: &EntityMap) -> Vec<EntityID> {
      ids.iter().map(|&id| map.match_source(id)).collect()
    }

    let soils = convert_ids(&self.seeds, &self.seed_to_soil);
    let fertilizers = convert_ids(&soils, &self.soil_to_fertilizer);
    let waters = convert_ids(&fertilizers, &self.fertilizer_to_water);
    let lights = convert_ids(&waters, &self.water_to_light);
    let temperatures = convert_ids(&lights, &self.light_to_temperature);
    let humidities = convert_ids(&temperatures, &self.temperature_to_humidity);
    let locations = convert_ids(&humidities, &self.humidity_to_location);

    locations
  }
}

impl AlmanacRange {

  pub fn convert_seeds_to_locations(&self) -> Range {

    let soils = self.seed_to_soil.match_range(self.seeds.clone());
    let fertilizers = self.soil_to_fertilizer.match_range(soils);
    let waters = self.fertilizer_to_water.match_range(fertilizers);
    let lights = self.water_to_light.match_range(waters);
    let temperatures = self.light_to_temperature.match_range(lights);
    let humidities = self.temperature_to_humidity.match_range(temperatures);
    let locations = self.humidity_to_location.match_range(humidities);

    locations
  }
}