export const ORTHOGRAPHY_STORAGE_KEY = 'varnavinyas-orthography-mode';
export const DEFAULT_ORTHOGRAPHY_MODE = 'academy-strict';

function supportedMode(mode) {
  return mode === 'common-editorial' || mode === 'academy-strict';
}

export function loadOrthographyMode(storage) {
  try {
    const mode = storage.getItem(ORTHOGRAPHY_STORAGE_KEY);
    return supportedMode(mode) ? mode : DEFAULT_ORTHOGRAPHY_MODE;
  } catch {
    return DEFAULT_ORTHOGRAPHY_MODE;
  }
}

export function saveOrthographyMode(storage, mode) {
  if (!supportedMode(mode)) return;
  try {
    storage.setItem(ORTHOGRAPHY_STORAGE_KEY, mode);
  } catch {
    // The selected mode still works when browser storage is unavailable.
  }
}

export function orthographyModeNote(mode) {
  return mode === 'common-editorial'
    ? 'प्रचलित लेखनमा संघीय, कांग्रेस र संकेत जस्ता समीक्षा गरिएका रूप वैकल्पिक हुन्। कडा रूप रोज्न सकिन्छ; सबै त्रुटि सच्याउने कार्यले यी रूप बदल्दैन। अन्य वर्णविन्यास त्रुटिको जाँच उस्तै रहन्छ।'
    : 'कडा जाँचमा संघीय, कांग्रेस र संकेतलाई त्रुटि मानी सङ्घीय, काङ्ग्रेस र सङ्केत सुझाइन्छ। सङ्घीय र सङ्केत प्रज्ञाका उदाहरण हुन्; काङ्ग्रेस शब्दकोशीय रूप हो।';
}
