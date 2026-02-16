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
    tooltip: 'उत्पत्तिअनुसार इ/ई, उ/ऊ को ह्रस्व/दीर्घ प्रयोग',
    summary:
      'तत्सम शब्दमा मूल दीर्घ/ह्रस्व संरचना कायम राखिन्छ। तद्भव, झर्रा र आगन्तुक शब्दमा नेपाली प्रयोगअनुसार ह्रस्व/दीर्घ लेखिन्छ। उपसर्गजन्य, द्वि/त्रि-पूर्व, र धेरै आगन्तुक शब्दमा ह्रस्व लेखन प्रचलित हुन्छ।',
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
    tooltip: 'तत्सममा शिरबिन्दु/पञ्चम, तद्भव-देशजमा चन्द्रबिन्दु',
    summary:
      'तत्सम शब्दमा शिरबिन्दु (ं) र पञ्चम वर्ण (ङ, ञ, ण, न, म) संस्कृत संरचनाअनुसार प्रयोग हुन्छ। तद्भव/देशज शब्दमा नासिक्य उच्चारणका लागि प्रायः चन्द्रबिन्दु (ँ) प्रयोग गरिन्छ। शब्दको उत्पत्तिअनुसार निर्णय गर्नुपर्छ।',
    examples: [
      { wrong: 'आंखा', correct: 'आँखा' },
      { wrong: 'गाँधी', correct: 'गान्धी' },
      { wrong: 'सन्सार', correct: 'संसार' },
      { wrong: 'बांस', correct: 'बाँस' },
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
    tooltip: 'श/ष/स को प्रयोग शब्दको उत्पत्तिअनुसार',
    summary:
      'श, ष, स उस्तै उच्चारणजस्ता देखिए पनि उत्पत्तिअनुसार फरक लेखिन्छन्। तत्सममा मूल रूप कायम राखिन्छ (जस्तै श/ष), तद्भव र अन्यमा प्रचलित नेपाली रूप मान्य हुन्छ।',
    examples: [
      { wrong: 'सान्ति', correct: 'शान्ति' },
      { wrong: 'बिसेष', correct: 'विशेष' },
      { wrong: 'बिशेष', correct: 'विशेष' },
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
    tooltip: 'ऋ तत्सममा; तद्भव/आगन्तुकमा रि/कृ वा प्रचलित रूप',
    summary:
      'ऋ/ृ संरचना तत्सम शब्दमा जस्ताको तस्तै राखिन्छ (ऋषि, ऋण, कृति)। तद्भव, देशज र आगन्तुक शब्दमा प्रायः रि/कृ वा चल्तीको नेपाली रूप प्रयोग हुन्छ।',
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
    tooltip: 'हलन्त र अजन्त प्रयोग शब्दरूप/क्रियारूपअनुसार',
    summary:
      'हलन्त (खुट्टा काट्ने) र अजन्त (नकाट्ने) प्रयोगले अर्थ र मानक रूप दुवैमा प्रभाव पार्छ। धातु, केही क्रियारूप र मान्/वान्/वत् प्रत्यययुक्त शब्दमा हलन्त लाग्छ; धेरै सर्वनाम, अव्यय, र सामान्य समापक क्रियारूप अजन्त लेखिन्छन्।',
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
    title: 'क्ष/छ/छ्य र ज्ञ/ग्याँ/ग्या भेद नियम',
    categoryCode: 'KshaChhya',
    tooltip: 'तत्सममा क्ष/ज्ञ, अन्यमा छ/छे/छ्य वा ग्याँ/ग्या',
    summary:
      'तत्सम शब्दमा क्ष र ज्ञ संयुक्त व्यञ्जन कायम राखिन्छ (क्षेत्र, ज्ञान, विज्ञान)। तद्भव/देशज/आगन्तुक शब्दमा छ/छे/छ्य वा ग्याँ/ग्या प्रचलित हुन सक्छ (जस्तै ग्याँस, ग्यारेज)।',
    examples: [
      { wrong: 'छत्रिय', correct: 'क्षत्रिय' },
      { wrong: 'छमा', correct: 'क्षमा' },
      { wrong: 'छेत्र', correct: 'क्षेत्र' },
      { wrong: 'अग्यान', correct: 'अज्ञान' },
      { wrong: 'प्रग्या', correct: 'प्रज्ञा' },
    ],
    subRules: [
      'तत्सममा क्ष/ज्ञ कायम',
      'तद्भव/आगन्तुकमा छे/छ्य वा ग्याँ/ग्या हुन सक्छ',
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
      { wrong: 'नि + शुल्क', correct: 'निःशुल्क' },
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
    tooltip: 'शब्द तालिका, पदयोग/पदवियोग, र प्रयोगगत सुधार',
    summary:
      'प्रज्ञा-प्रतिष्ठानको शुद्ध/अशुद्ध सूची, पदयोग/पदवियोग नियम, र पद-प्रयोगगत सुधार यस खण्डमा पर्छन्। शब्दकोशीय रूप, जोडेर/छुट्याएर लेखाइ, र प्रचलित अशुद्ध रूपहरूको मानकीकरण यही आधारमा गरिन्छ।',
    examples: [
      { wrong: 'प्रसाशन', correct: 'प्रशासन' },
      { wrong: 'संघीय', correct: 'सङ्घीय' },
      { wrong: 'आज्ञा अनुसार', correct: 'आज्ञाअनुसार' },
      { wrong: 'तिमी भन्दा', correct: 'तिमीभन्दा' },
      { wrong: 'उपरोक्त', correct: 'उपर्युक्त' },
    ],
    subRules: [
      'शुद्ध-अशुद्ध पदसूचीअनुसार मानक रूप चयन',
      'उपसर्ग, प्रत्यय, विभक्ति, नामयोगीमा पदयोग',
      'निपात, केही क्रियारूप र पूर्ण द्वित्वमा पदवियोग',
      'कहिलेकाहीँ अर्थभेदका कारण शैलीगत सुझाव मात्र हुन सक्छ',
    ],
  },
  {
    title: 'विराम चिह्न नियम',
    categoryCode: 'Punctuation',
    tooltip: 'Section 5: विराम/उद्धरण/निर्देशक/सङ्क्षेप/ऐजन चिह्न',
    summary:
      'नेपाली लेखनमा अल्पविराम, अर्धविराम, पूर्णविराम, प्रश्नवाचक, उद्गार, निर्देशक/सापेक्षविराम (: / - / :-), उद्धरण, कोष्ठक, योजक, तिर्यक् विराम, सङ्क्षेप (.) र ऐजन (,,/…) को सन्दर्भअनुसार प्रयोग गर्नुपर्छ।',
    examples: [
      { wrong: 'नेपाल सुन्दर छ.', correct: 'नेपाल सुन्दर छ।' },
      { wrong: '"उनले भने"', correct: '\u201cउनले भने\u201d' },
      { wrong: 'अ. दु. अ .आ.', correct: 'अ. दु. अ. आ.' },
      { wrong: ', ,', correct: ',,' },
      { wrong: '...', correct: '\u2026' },
    ],
    subRules: [
      'अल्पविराम (,)',
      'अर्धविराम (;)',
      'पूर्णविराम (।)',
      'प्रश्नचिह्न (?)',
      'विस्मयचिह्न (!)',
      'निर्देशक/सापेक्षविराम (: / - / :-)',
      'उद्धरण (\' \' / “ ”) र कोष्ठक ( )',
      'योजक (-) र तिर्यक् विराम (/)',
      'सङ्क्षेप (.) र ऐजन (,, / …)',
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
  Chandrabindu: ['चन्द्रबिन्दु', 'शिरबिन्दु', 'अनुस्वार', 'पञ्चम'],
  ShaShaS: ['श/ष/स', 'ऊष्म', 'ष'],
  RiKri: ['ऋ/कृ', 'ऋकार', 'रि/ऋ'],
  Halanta: ['हलन्त', 'halanta'],
  KshaChhya: ['क्ष/छ', 'क्ष', 'छ्य', 'ज्ञ', 'ग्याँ', 'ग्या'],
  YaE: ['य/ए'],
  Sandhi: ['सन्धि', 'sandhi'],
  ShuddhaTable: ['शुद्ध', 'अशुद्ध', 'तालिका', 'पदयोग', 'पदवियोग', 'section4-phrase-style'],
  Punctuation: ['विराम', 'चिह्न', 'punctuation', 'निर्देशक', 'सापेक्षविराम', 'सङ्क्षेप', 'ऐजन', ':-'],
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
    return `<span class="rule-ref" tabindex="0" role="button" aria-label="${escapeHtml(tooltip)}" data-tooltip="${escapeHtml(tooltip)}" data-category="${escapeHtml(cat)}">${escapeHtml(ruleText)}</span>`;
  }
  if (tooltip) {
    return `<span class="rule-ref" tabindex="0" role="button" aria-label="${escapeHtml(tooltip)}" data-tooltip="${escapeHtml(tooltip)}">${escapeHtml(ruleText)}</span>`;
  }
  return escapeHtml(ruleText);
}
