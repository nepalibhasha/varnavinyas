# varnavinyas-samasa

Conservative compound interpretations for public orthography tools.

`analyze_compound(word)` returns reviewed interpretations of an uninflected word
with the existing `left`, `right`, `samasa_type`, `score`, and `vigraha` fields.
An empty result means that no supported interpretation is available; it does not
mean that the word is misspelled or has no historical derivation.

## Evidence and scope

`data/rule_inventories/samasa.tsv` supplies the reviewed word, members, type,
vigraha, source and review status. Parser tests reject missing provenance,
unreviewed/duplicate rows and self-repeating members. Lexical lookup and exact
forward composition also validate every pair. Dictionary membership, a sandhi
round trip, numeral membership or POS labels alone cannot establish samasa.

The initial reviewed inventory has five interpretations: सूर्योदय, महोत्सव,
एकचक्र (the numeral reading), पूर्वाधार and मापदण्ड. महोत्सव uses महा + उत्सव,
matching the dictionaries' large-festival meaning. पूर्वाधार uses the
adjectival पहिलेको sense of पूर्व, rather than a fabricated पूर्व को आधार.
This is bounded coverage, not a general compound parser or contextual sense
selection. Growing coverage requires reviewed evidence for each interpretation.

`score` retains legacy ranking weights for these interpretations; it is not a
calibrated probability and should not be presented as an accuracy percentage.
Types and vigraha describe a supported reading, not every possible use of a word.

Prefix/suffix derivations belong to morphology, rather than automatic samasa
classification. `sandhi::split` and `split_best_for_compound` remain exploratory
spelling-boundary APIs; their candidates are not proof of a compound relationship.
The checker analyzes the supported stem and explicitly preserves outer suffixes
in informational output, for example सूर्योदयमा → सूर्य + उदय + मा.

See [consumer guidance](../../docs/COMPOUND_ANALYSIS.md).
