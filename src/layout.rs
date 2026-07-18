const RU_TO_EN_LOWER: &[(char, char)] = &[
    ('й', 'q'), ('ц', 'w'), ('у', 'e'), ('к', 'r'), ('е', 't'),
    ('н', 'y'), ('г', 'u'), ('ш', 'i'), ('щ', 'o'), ('з', 'p'),
    ('х', '['), ('ъ', ']'), ('ф', 'a'), ('ы', 's'), ('в', 'd'),
    ('а', 'f'), ('п', 'g'), ('р', 'h'), ('о', 'j'), ('л', 'k'),
    ('д', 'l'), ('ж', ';'), ('э', '\''), ('я', 'z'), ('ч', 'x'),
    ('с', 'c'), ('м', 'v'), ('и', 'b'), ('т', 'n'), ('ь', 'm'),
    ('б', ','), ('ю', '.'), ('.', '/'), ('ё', '`'),
];
const RU_TO_EN_UPPER: &[(char, char)] = &[
    ('Й', 'Q'), ('Ц', 'W'), ('У', 'E'), ('К', 'R'), ('Е', 'T'),
    ('Н', 'Y'), ('Г', 'U'), ('Ш', 'I'), ('Щ', 'O'), ('З', 'P'),
    ('Х', '{'), ('Ъ', '}'), ('Ф', 'A'), ('Ы', 'S'), ('В', 'D'),
    ('А', 'F'), ('П', 'G'), ('Р', 'H'), ('О', 'J'), ('Л', 'K'),
    ('Д', 'L'), ('Ж', ':'), ('Э', '"'), ('Я', 'Z'), ('Ч', 'X'),
    ('С', 'C'), ('М', 'V'), ('И', 'B'), ('Т', 'N'), ('Ь', 'M'),
    ('Б', '<'), ('Ю', '>'), (',', '?'), ('Ё', '~'),
];

fn map_char(c: char, table: &[(char, char)]) -> Option<char> {
    table.iter().find(|&&(ru, _)| ru == c).map(|&(_, en)| en)
}

fn map_char_rev(c: char, table: &[(char, char)]) -> Option<char> {
    table.iter().find(|&&(_, en)| en == c).map(|&(ru, _)| ru)
}

fn is_in_table(c: char, table: &[(char, char)]) -> bool {
    table.iter().any(|&(ru, _)| ru == c)
}

fn is_in_table_rev(c: char, table: &[(char, char)]) -> bool {
    table.iter().any(|&(_, en)| en == c)
}

pub fn to_russian(text: &str) -> String {
    text.chars()
        .map(|c| map_char_rev(c, RU_TO_EN_LOWER).or_else(|| map_char_rev(c, RU_TO_EN_UPPER)).unwrap_or(c))
        .collect()
}

pub fn to_english(text: &str) -> String {
    text.chars()
        .map(|c| map_char(c, RU_TO_EN_LOWER).or_else(|| map_char(c, RU_TO_EN_UPPER)).unwrap_or(c))
        .collect()
}

pub fn toggle_layout(text: &str) -> String {
    let ru_count = text.chars().filter(|c| is_in_table(*c, RU_TO_EN_LOWER) || is_in_table(*c, RU_TO_EN_UPPER)).count();
    let en_count = text.chars().filter(|c| is_in_table_rev(*c, RU_TO_EN_LOWER) || is_in_table_rev(*c, RU_TO_EN_UPPER)).count();
    if ru_count > en_count {
        to_english(text)
    } else {
        to_russian(text)
    }
}
