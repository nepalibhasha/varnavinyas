/**
 * Rules reference data — Nepal Academy orthography standard sections.
 *
 * Each section maps to a category_code used in diagnostics.
 * `tooltip` is the one-line hover summary; `summary` is the full explanation.
 */
import { escapeHtml } from './utils.js';

export const RULES_SECTIONS = [
  {
    title: 'ह्रस्व/दीर्घ स्वर नियम',
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
      'तत्सम शब्दमा मूल दीर्घ कायम',
      'तद्भव शब्दमा ह्रस्व प्रयोग',
      'देशज शब्दमा ह्रस्व प्रयोग',
      'आगन्तुक शब्दमा ह्रस्व प्रयोग',
    ],
  },
  {
    title: 'चन्द्रबिन्दु/शिरबिन्दु नियम',
    categoryCode: 'Chandrabindu',
    tooltip: 'तत्सममा शिरबिन्दु+पञ्चम, तद्भवमा चन्द्रबिन्दु',
    summary:
      'तत्सम शब्दमा अनुस्वार (शिरबिन्दु ं) र पञ्चम वर्ण (ङ्, ञ्, ण्, न्, म्) संस्कृत नियमअनुसार प्रयोग गर्ने। तद्भव/देशज शब्दमा नासिक्य ध्वनिका लागि चन्द्रबिन्दु (ँ) प्रयोग गर्ने।',
    examples: [
      { wrong: 'आंखा', correct: 'आँखा' },
      { wrong: 'गाँधी', correct: 'गान्धी' },
      { wrong: 'सन्सार', correct: 'संसार' },
      { wrong: 'हंस', correct: 'हँस' },
    ],
    subRules: [
      'तत्सममा पञ्चम वर्ण + अनुस्वार',
      'तद्भवमा चन्द्रबिन्दु',
      'वर्गीय नासिक्यमा पञ्चम वर्ण',
    ],
  },
  {
    title: 'श/ष/स प्रयोग नियम',
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
      'तत्सममा ष कायम',
      'तद्भवमा स/श प्रयोग',
      'ऋ तत्सम शब्दमा मात्र',
    ],
  },
  {
    title: 'ऋ/कृ प्रयोग नियम',
    categoryCode: 'RiKri',
    tooltip: 'ऋ तत्सम शब्दमा मात्र; अन्यत्र कृ/रि',
    summary:
      'ऋ स्वरको प्रयोग तत्सम शब्दमा मात्र हुन्छ (ऋण, ऋतु, कृषि)। तद्भव/देशज/आगन्तुक शब्दमा ऋ को सट्टा रि वा कृ प्रयोग गर्ने।',
    examples: [
      { wrong: 'ऋँगटा', correct: 'रिँगटा' },
      { wrong: 'ऋणी', correct: 'ऋणी' },
    ],
    subRules: [
      'तत्सममा ऋ कायम',
      'तद्भवमा रि/कृ',
    ],
  },
  {
    title: 'हलन्त नियम',
    categoryCode: 'Halanta',
    tooltip: 'संयुक्त व्यञ्जन नबन्ने ठाउँमा हलन्त अनिवार्य',
    summary:
      'व्यञ्जनको अन्तमा स्वर नभएमा हलन्त (्) चिह्न लगाउनुपर्छ। संयुक्ताक्षर बन्न नसक्ने स्थानमा हलन्त अनिवार्य। तर संयुक्ताक्षर बन्ने ठाउँमा हलन्तको सट्टा संयुक्ताक्षर नै लेख्ने।',
    examples: [
      { wrong: 'महान', correct: 'महान्' },
      { wrong: 'जगत', correct: 'जगत्' },
    ],
    subRules: [
      'शब्दान्तमा हलन्त',
      'संयुक्ताक्षर बन्ने ठाउँमा हलन्त नलगाउने',
    ],
  },
  {
    title: 'क्ष/छ भेद नियम',
    categoryCode: 'KshaChhya',
    tooltip: 'तत्सममा क्ष, तद्भवमा छ',
    summary:
      'तत्सम शब्दमा क्ष (क्+ष) प्रयोग गर्ने। तद्भव/देशज शब्दमा जहाँ क्ष को अपभ्रंश भएको छ, त्यहाँ छ लेख्ने।',
    examples: [
      { wrong: 'छत्रिय', correct: 'क्षत्रिय' },
      { wrong: 'छमा', correct: 'क्षमा' },
      { wrong: 'छेत्र', correct: 'क्षेत्र' },
    ],
    subRules: [
      'तत्सममा क्ष कायम',
      'तद्भवमा छ प्रयोग',
    ],
  },
  {
    title: 'य/ए भेद नियम',
    categoryCode: 'YaE',
    tooltip: 'शब्दादिमा य र ए को सही प्रयोग',
    summary:
      'तत्सम शब्दमा य (यज्ञ, यथार्थ) र एक-मूलका शब्दमा ए (एक, एकता) प्रयोग हुन्छ। शब्दको सुरुमा य र ए को भेद राख्ने।',
    examples: [
      { wrong: 'एथार्थ', correct: 'यथार्थ' },
      { wrong: 'यकता', correct: 'एकता' },
    ],
    subRules: [
      'तत्सममा य प्रयोग (यज्ञ, यथार्थ)',
      'एक-मूलका शब्दमा ए प्रयोग (एक, एकता)',
    ],
  },
  {
    title: 'सन्धि नियम',
    categoryCode: 'Sandhi',
    tooltip: 'स्वर/विसर्ग/व्यञ्जन सन्धि नियम',
    summary:
      'नेपाली वर्णविन्यासमा सन्धि — स्वर सन्धि, विसर्ग सन्धि, र व्यञ्जन सन्धि — संस्कृत व्याकरणको नियमअनुसार गर्ने। तत्सम शब्दमा सन्धि कायम, तद्भवमा प्रचलनअनुसार।',
    examples: [
      { wrong: 'अत्याधिक', correct: 'अत्यधिक' },
      { wrong: 'कवि + इन्द्र', correct: 'कवीन्द्र' },
      { wrong: 'देव + ऋषि', correct: 'देवर्षि' },
      { wrong: 'वाक् + दान', correct: 'वाग्दान' },
      { wrong: 'अन्तः + तल', correct: 'अन्तःतल' },
    ],
    subRules: [
      'स्वर सन्धि (दीर्घ, गुण, वृद्धि, यण्, अयादि)',
      'विसर्ग सन्धि',
      'व्यञ्जन सन्धि (स्वरीकरण, अनुनासिकीकरण, समीकरण)',
    ],
  },
  {
    title: 'शुद्ध-अशुद्ध शब्द तालिका',
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
    title: 'विराम चिह्न नियम',
    categoryCode: 'Punctuation',
    tooltip: 'देवनागरी विराम चिह्नका १४ प्रकार',
    summary:
      'नेपाली लेखनमा १४ प्रकारका विराम चिह्न प्रयोग हुन्छन्: पूर्णविराम (।), अर्धविराम (;), अल्पविराम (,), प्रश्नचिह्न (?), विस्मयचिह्न (!), उद्धरणचिह्न (""), कोष्ठक (()), योजकचिह्न (-), आदि।',
    examples: [
      { wrong: 'नेपाल सुन्दर छ.', correct: 'नेपाल सुन्दर छ।' },
      { wrong: '"उनले भने"', correct: '\u201cउनले भने\u201d' },
    ],
    subRules: [
      'पूर्णविराम (।)',
      'अर्धविराम (;)',
      'अल्पविराम (,)',
      'प्रश्नचिह्न (?)',
      'विस्मयचिह्न (!)',
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
    if (ruleText.includes(section.title)) {
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
 */
export function getTooltipForRule(ruleText) {
  const cat = getCategoryForRule(ruleText);
  return cat ? (RULE_TOOLTIPS[cat] || null) : null;
}

/**
 * Wrap a rule citation in a tooltip-enabled span.
 * Shared by checker.js and inspector.js.
 */
export function wrapRuleTooltip(ruleText, categoryCode) {
  const cat = categoryCode || getCategoryForRule(ruleText);
  const tooltip = (cat && RULE_TOOLTIPS[cat]) || getTooltipForRule(ruleText);
  if (tooltip && cat) {
    return `<span class="rule-ref" data-tooltip="${escapeHtml(tooltip)}" data-category="${escapeHtml(cat)}">${escapeHtml(ruleText)}</span>`;
  }
  if (tooltip) {
    return `<span class="rule-ref" data-tooltip="${escapeHtml(tooltip)}">${escapeHtml(ruleText)}</span>`;
  }
  return escapeHtml(ruleText);
}
