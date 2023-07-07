import type { FeatureCollection } from "geojson";

export function fixComparisonData(gj: FeatureCollection): FeatureCollection {
  // https://cycling.data.tfl.gov.uk/CyclingInfrastructure/documentation/asset_information_guide.pdf as reference
  let errors = 0;
  for (let f of gj.features) {
    // TODO For now, don't enforce these checks; just log violations
    // The rules aren't correct yet!
    try {
      f.properties.kind = classify(f.properties);
      // Replace old values with the consolidated enum
      for (let key of [
        "CLT_CARR",
        "CLT_SEGREG",
        "CLT_STEPP",
        "CLT_PARSEG",
        "CLT_SHARED",
        "CLT_MANDAT",
        "CLT_ADVIS",
      ]) {
        delete f.properties[key];
      }
    } catch (err) {
      console.log(err);
      errors++;
    }

    // Delete properties that aren't helpful to conflate yet
    for (let key of [
      "BOROUGH",
      "CLT_PRIORI",
      "CLT_CBYPAS",
      "CLT_BBYPAS",
      "CLT_PARKR",
      "CLT_WATERR",
      "CLT_PTIME",
      "CLT_ACCESS",
    ]) {
      delete f.properties[key];
    }
  }
  console.log(`${errors} errors total`);
  return gj;
}

function classify(props: Record<string, string>): string {
  // Find the true features from a list
  let trues = [];
  for (let key of [
    "CLT_SEGREG",
    "CLT_STEPP",
    "CLT_PARSEG",
    "CLT_SHARED",
    "CLT_MANDAT",
    "CLT_ADVIS",
  ]) {
    if (props[key]) {
      trues.push(key);
    }
  }
  let onCarriageway = props.CLT_CARR;
  let contraflow = props.CLT_CONTRA;

  if (trues.length == 0 && contraflow) {
    return "contraflow";
  }

  // Expect exactly one to be true
  if (trues.length != 1) {
    throw new Error(
      `Figuring out the enum failed for ${JSON.stringify(props)}`
    );
  }

  let kind = null;
  if (onCarriageway) {
    kind = {
      CLT_SEGREG: "fully segregated",
      CLT_STEPP: "stepped",
      CLT_PARSEG: "partial segregation",
      CLT_SHARED: "shared bus lane",
      CLT_MANDAT: "mandatory cycle lane",
      CLT_ADVIS: "advisory cycle lane",
    }[trues[0]];
  } else {
    kind = {
      CLT_SEGREG: "fully segregated off carriageway",
      CLT_STEPP: null,
      CLT_PARSEG: "partial segregation off carriageway",
      CLT_SHARED: "shared footway",
      CLT_MANDAT: null,
      CLT_ADVIS: null,
    }[trues[0]];
  }

  if (!kind) {
    throw new Error(
      `Figuring out the enum failed for ${JSON.stringify(props)}`
    );
  }

  return kind;
}
