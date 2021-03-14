# gen_declension_table
Simple command line application that takes the Open Corpora (http://opencorpora.org/) dict.opcorpora.xml and generates a sqlite db with a single table "nouns", containing the nouns found there and their declensions by case (also it's gender and if it's animate or not).

The generated database (ru_practice.db) is therefore a derivative of the file dict.opcorpora.xml from http://opencorpora.org, which was used under CC-BY-SA 3.0. "ru_practice.db" is also licenced under CC-BY-SA 3.0.

This database is the one used in the project "ru_practice" (https://github.com/brianch/ru_practice).
