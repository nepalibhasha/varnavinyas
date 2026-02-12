/**
 * Rules reference data — Nepal Academy orthography standard sections.
 *
 * Each section maps to a category_code used in diagnostics.
 * `tooltip` is the one-line hover summary; `summary` is the full explanation.
 */

export const RULES_SECTIONS = [
  {
    code: '3(क)',
    title: 'ह्रस्व/दीर्घ स्वर',
    categoryCode: 'HrasvaDirgha',
    tooltip: 'तत्सममा दीर्घ, तद्भव/देशज/आगन्तुकमा ह्रस्व प्रयोग गर्ने',
    summary:
      'तत्सम (संस्कृत मूलका) शब्दमा संस्कृतको मूल वर्णविन्यास कायम राख्नुपर्छ — दीर्घ ई, ऊ जस्ताको तस्तै। तद्भव, देशज र आगन्तुक शब्दमा ह्रस्व इ, उ प्रयोग गर्नुपर्छ।',
    examples: [
      { wrong: 'बिमारी', correct: 'बिमारि' },
      { wrong: 'दुरी', correct: 'दुरि' },
      { wrong: 'नदि', correct: 'नदी' },
      { wrong: 'लक्ष्मि', correct: 'लक्ष्मी' },
    ],
    subRules: [
      '3(क)-1: तत्सम शब्दमा मूल दीर्घ कायम',
      '3(क)-2: तद्भव शब्दमा ह्रस्व प्रयोग',
      '3(क)-3: देशज शब्दमा ह्रस्व प्रयोग',
      '3(क)-4: आगन्तुक शब्दमा ह्रस्व प्रयोग',
    ],
  },
  {
    code: '3(ख)',
    title: 'अनुस्वार/चन्द्रबिन्दु/पञ्चम वर्ण',
    categoryCode: 'Chandrabindu',
    tooltip: 'तत्सममा शिरबिन्दु+पञ्चम, तद्भवमा चन्द्रबिन्दु',
    summary:
      'तत्सम शब्दमा अनुस्वार (शिरबिन्दु ं) र पञ्चम वर्ण (ङ्, ञ्, ण्, न्, म्) संस्कृत नियमअनुसार प्रयोग गर्ने। तद्भव/देशज शब्दमा नासिक्य ध्वनिका लागि चन्द्रबिन्दु (ँ) प्रयोग गर्ने।',
    examples: [
      { wrong: 'आंखा', correct: 'आँखा' },
      { wrong: 'गाँधी', correct: 'गान्धी' },
      { wrong: 'सन्सार', correct: 'संसार' },
      { wrong: 'हंस', correct: 'हंस' },
    ],
    subRules: [
      '3(ख)-1: तत्सममा पञ्चम वर्ण + अनुस्वार',
      '3(ख)-2: तद्भवमा चन्द्रबिन्दु',
      '3(ख)-3: वर्गीय नासिक्यमा पञ्चम वर्ण',
    ],
  },
  {
    code: '3(ग)',
    title: 'श/ष/स विधान र ऋ/कृ',
    categoryCode: 'ShaShaS',
    tooltip: 'श/ष/स उत्पत्ति अनुसार; ऋ तत्सममा मात्र',
    summary:
      'श, ष, स — यी तीन ऊष्म व्यञ्जन शब्दको उत्पत्ति अनुसार प्रयोग गर्ने। तत्सम शब्दमा संस्कृत मूलअनुसार ष प्रयोग। ऋ स्वर तत्सम शब्दमा मात्र; अन्यमा कृ/रि प्रयोग।',
    examples: [
      { wrong: 'सान्ति', correct: 'शान्ति' },
      { wrong: 'बिसेष', correct: 'विशेष' },
      { wrong: 'कृषि', correct: 'कृषि' },
      { wrong: 'ऋण', correct: 'ऋण' },
    ],
    subRules: [
      '3(ग)-1: तत्सममा ष कायम',
      '3(ग)-2: तद्भवमा स/श प्रयोग',
      '3(ग)-3: ऋ तत्सम शब्दमा मात्र',
    ],
  },
  {
    code: '3(ग)',
    title: 'ऋ/कृ नियम',
    categoryCode: 'RiKri',
    tooltip: 'ऋ तत्सम शब्दमा मात्र; अन्यत्र कृ/रि',
    summary:
      'ऋ स्वरको प्रयोग तत्सम शब्दमा मात्र हुन्छ (ऋण, ऋतु, कृषि)। तद्भव/देशज/आगन्तुक शब्दमा ऋ को सट्टा रि वा कृ प्रयोग गर्ने।',
    examples: [
      { wrong: 'ऋँगटा', correct: 'रिँगटा' },
      { wrong: 'ऋणी', correct: 'ऋणी' },
    ],
    subRules: [
      '3(ग)-ऋ-1: तत्सममा ऋ कायम',
      '3(ग)-ऋ-2: तद्भवमा रि/कृ',
    ],
  },
  {
    code: '3(ङ)',
    title: 'हलन्त विधान',
    categoryCode: 'Halanta',
    tooltip: 'संयुक्त व्यञ्जन नबन्ने ठाउँमा हलन्त अनिवार्य',
    summary:
      'व्यञ्जनको अन्तमा स्वर नभएमा हलन्त (्) चिह्न लगाउनुपर्छ। संयुक्ताक्षर बन्न नसक्ने स्थानमा हलन्त अनिवार्य। तर संयुक्ताक्षर बन्ने ठाउँमा हलन्तको सट्टा संयुक्ताक्षर नै लेख्ने।',
    examples: [
      { wrong: 'महान', correct: 'महान्' },
      { wrong: 'जगत', correct: 'जगत्' },
    ],
    subRules: [
      '3(ङ)-1: शब्दान्तमा हलन्त',
      '3(ङ)-2: संयुक्ताक्षर बन्ने ठाउँमा हलन्त नलगाउने',
    ],
  },
  {
    code: '3(छ)',
    title: 'क्ष/छ नियम',
    categoryCode: 'KshaChhya',
    tooltip: 'तत्सममा क्ष, तद्भवमा छ',
    summary:
      'तत्सम शब्दमा क्ष (क्+ष) प्रयोग गर्ने। तद्भव/देशज शब्दमा जहाँ क्ष को अपभ्रंश भएको छ, त्यहाँ छ लेख्ने।',
    examples: [
      { wrong: 'छत्रिय', correct: 'क्षत्रिय' },
      { wrong: 'क्षमा', correct: 'क्षमा' },
      { wrong: 'क्षेत्र', correct: 'क्षेत्र' },
    ],
    subRules: [
      '3(छ)-1: तत्सममा क्ष कायम',
      '3(छ)-2: तद्भवमा छ प्रयोग',
    ],
  },
  {
    code: '3(च)',
    title: 'य/ए नियम',
    categoryCode: 'YaE',
    tooltip: 'सन्धि/प्रत्ययमा य र ए को सही प्रयोग',
    summary:
      'सन्धि र प्रत्ययमा य र ए को भेद राख्ने। शब्दको बीचमा स्वर सन्धिमा "य" प्रयोग हुन्छ, तर अन्तमा "ए" प्रयोग हुने ठाउँमा ए नै लेख्ने।',
    examples: [
      { wrong: 'राजनैतिक', correct: 'राजनीतिक' },
      { wrong: 'दैनिक', correct: 'दैनिक' },
    ],
    subRules: [
      '3(च)-1: सन्धिमा य प्रयोग',
      '3(च)-2: प्रत्ययमा ए कायम',
    ],
  },
  {
    code: '3(घ)',
    title: 'सन्धि नियम',
    categoryCode: 'Sandhi',
    tooltip: 'स्वर/विसर्ग/व्यञ्जन सन्धि नियम',
    summary:
      'नेपाली वर्णविन्यासमा सन्धि — स्वर सन्धि, विसर्ग सन्धि, र व्यञ्जन सन्धि — संस्कृत व्याकरणको नियमअनुसार गर्ने। तत्सम शब्दमा सन्धि कायम, तद्भवमा प्रचलनअनुसार।',
    examples: [
      { wrong: 'अत्याधिक', correct: 'अत्यधिक' },
      { wrong: 'सदैव', correct: 'सदैव' },
    ],
    subRules: [
      '3(घ)-1: स्वर सन्धि',
      '3(घ)-2: विसर्ग सन्धि',
      '3(घ)-3: व्यञ्जन सन्धि',
    ],
  },
  {
    code: '४',
    title: 'शुद्ध-अशुद्ध तालिका',
    categoryCode: 'ShuddhaTable',
    tooltip: 'नेपाल प्रज्ञा प्रतिष्ठानको शुद्ध/अशुद्ध शब्द तालिका',
    summary:
      'नेपाल प्रज्ञा प्रतिष्ठानले प्रकाशित गरेको शुद्ध-अशुद्ध शब्द तालिकामा करिब २००० भन्दा बढी शब्दजोडी छन्। यो तालिकाले सही वर्णविन्यासको प्रामाणिक सन्दर्भ प्रदान गर्छ।',
    examples: [
      { wrong: 'प्रसाशन', correct: 'प्रशासन' },
      { wrong: 'अत्याधिक', correct: 'अत्यधिक' },
      { wrong: 'व्यवस्थित', correct: 'व्यवस्थित' },
      { wrong: 'सामाजिक', correct: 'सामाजिक' },
    ],
    subRules: [],
  },
  {
    code: '५',
    title: 'विराम चिह्न',
    categoryCode: 'Punctuation',
    tooltip: 'देवनागरी विराम चिह्नका १४ प्रकार',
    summary:
      'नेपाली लेखनमा १४ प्रकारका विराम चिह्न प्रयोग हुन्छन्: पूर्णविराम (।), अर्धविराम (;), अल्पविराम (,), प्रश्नचिह्न (?), विस्मयचिह्न (!), उद्धरणचिह्न (""), कोष्ठक (()), योजकचिह्न (-), आदि।',
    examples: [
      { wrong: 'नेपाल सुन्दर छ.', correct: 'नेपाल सुन्दर छ।' },
      { wrong: '"उनले भने"', correct: '"उनले भने"' },
    ],
    subRules: [
      '५-1: पूर्णविराम (।)',
      '५-2: अर्धविराम (;)',
      '५-3: अल्पविराम (,)',
      '५-4: प्रश्नचिह्न (?)',
      '५-5: विस्मयचिह्न (!)',
    ],
  },
];

/**
 * Lookup: categoryCode → tooltip string.
 * Used by checker.js and deriver.js for hover tooltips on rule citations.
 */
export const RULE_TOOLTIPS = Object.fromEntries(
  RULES_SECTIONS.map((s) => [s.categoryCode, s.tooltip])
);

/**
 * Lookup: parse a rule citation string and return matching tooltip.
 * Handles formats like "वर्णविन्यास नियम: 3(क)(ई)" or "शुद्ध/अशुद्ध तालिका: ..."
 */
/** Keyword map shared by tooltip and category lookups. */
const RULE_KEYWORDS = {
  HrasvaDirgha: ['ह्रस्व', 'दीर्घ', 'hrasva', 'dirgha'],
  Chandrabindu: ['चन्द्रबिन्दु', 'अनुस्वार', 'पञ्चम'],
  ShaShaS: ['श/ष/स', 'ऊष्म'],
  RiKri: ['ऋ/कृ', 'ऋकार'],
  Halanta: ['हलन्त', 'halanta'],
  KshaChhya: ['क्ष/छ', 'क्ष'],
  YaE: ['य/ए'],
  Sandhi: ['सन्धि', 'sandhi'],
  ShuddhaTable: ['शुद्ध', 'अशुद्ध', 'तालिका'],
  Punctuation: ['विराम', 'चिह्न', 'punctuation'],
};

/**
 * Resolve a rule citation string to its categoryCode.
 */
export function getCategoryForRule(ruleText) {
  if (!ruleText) return null;

  for (const section of RULES_SECTIONS) {
    if (ruleText.includes(section.code) || ruleText.includes(section.title)) {
      return section.categoryCode;
    }
  }

  for (const [code, kws] of Object.entries(RULE_KEYWORDS)) {
    if (kws.some((kw) => ruleText.toLowerCase().includes(kw.toLowerCase()))) {
      return code;
    }
  }

  return null;
}

/**
 * Lookup: parse a rule citation string and return matching tooltip.
 * Handles formats like "वर्णविन्यास नियम: 3(क)(ई)" or "शुद्ध/अशुद्ध तालिका: ..."
 */
export function getTooltipForRule(ruleText) {
  const cat = getCategoryForRule(ruleText);
  return cat ? (RULE_TOOLTIPS[cat] || null) : null;
}
