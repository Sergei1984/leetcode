У цій статті розглянемо вирішення задачі з літкоду.
Щоб не брати щось просте, візьмемо задачу з підрозділу hard [Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/) і вирішимо його на мові програмування Rust.

Відразу скажу, що моє рішення не є оптимальним по швидкості чи по памʼяті, і ціль статті - показати ітеративно підходи до вирішення складних задач.

## Умови задачі і розгляд проблеми

> Given an input string s and a pattern p, implement regular expression matching with support for `.` and `*` where:
>
> - `.` Matches any single character.​​​​
> - `*` Matches zero or more of the preceding element.
>
>   The matching should cover the entire input string (not partial).

Тобто, нам треба реалізувати алгоритм співставлення строки з регулярним виразом спрощенного вигляду.

Маємо такий синтаксис:

- `a` - співставляється з одним символом `a`. Може бути будь який символ, окрім `.` та `*`, які мають спеціальне значення. Зверніть увагу, що екранування піддтримувати не потрібно, цього нема в умовах і рішення проходить тести і без нього.
- `.` - співставляється з будь яким _одним_ символом. Символ обовʼязково має бути присутній, пуста строка не рахується за співпадіння.
- `*` - квантифікатор "нуль або більше". Вказує на те, що попередній символ може зустрічатися від нуля і більше.

Також, суттєва умова - вираз має співпадати з усією строкою, а не з префіксом. Тобто, регулярка `a***b` не співпадає зі строкою `abc`. У цьому є відмінність від звичайних регулярок, які співставляються з підстрокою.

Хоча задача може здатися на вигляд дуже простою (як і здалося мені с першого погляду), вона не даремно знаходиться у розділі hard, як ми далі і побачимо.

Цю задачу складно навіть просто вирішити, і ще складніше вирішити ефективно. Складність її для мене була у тому, що є досить багато випадків, які, природьно, не указані в умовах, і до яких я доходив у процесі вирішення. Також ми побачимо, що алгоритм, який може здатися досить простим і прямолінійним, не є таким.

## Підготовка до вирішення

Підготуємося до вирішення.

Встановлення та налаштування Rust та середовища я не буду тут описувати, все стандартно. Створюємо новий проект і редагуємо файл `main.rs`

```
#[allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case001() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }
}

fn main() {
    println!("Hello, world!");
}

```

Запускаємо `cargo test` і бачимо, що наш тест пройшов.
Далі будемо додавати код, і, відповідно, додавати тест-кейси.

## Перший погляд на алгоритм і перша ітерація

По-перше, наш алгоритм рекурсивний. Тобто, ми беремо перший патерн з регулярного виразу, і співставляємо його з початком строки. Якщо вдалося співставити, викликаємо рекурсивно алгоритм для залишку регулярки і залишку вхідної строки. Синтаксис нашої регулярки досить простий, і ми можемо реалізувати алгоритм без look-back.

Створимо структуру для опису патерну. Тут, насправді, я трохи забігаю наперед, бо починав я вирішення без цієї структури, але задача статті не показати повністю хід моїх думок від початку до кінця, а показати, як ітеративно вирішувати складні задачі. Тому най буде.

```
pub struct Pattern<'a> {
    pub match_char: char,
    pub allow_multiple: bool,
    pub next_regex: &'a str,
}
```

Структура досить проста. `match_char` містить символ, з яким іде співставлення, або `.`. `allow_multiple` - якщо після символа була зірочка `*`, і `next_regex` - це залишок регулярки. Також маємо лайфтайм `'a`. Його потрібно вказувати, якщо структура у собі містить нестатичні посилання (`&`). Цей лайфтайм підкаже компілятору, що структура не має пережити посилання всередені неї (тобто дані, на які ми посилаємося, не можна видалити раніше, ніж структуру).

Також наведу функцію, яка створює цю структуру зі строки:

```
fn next_pattern<'a>(regex: &'a str) -> Option<Pattern<'a>> {
    let mut next_index = 1;
    let mut chars = regex.chars();

    let next_char = chars.next();
    if next_char.is_none() {
        return None;
    }

    let match_char = next_char.unwrap();

    let allow_multiple = chars.next().unwrap_or_default() == '*';
    if allow_multiple {
        next_index = next_index + 1;
    }

    Some(Pattern {
        match_char,
        allow_multiple,
        next_regex: &regex[next_index..],
    })
}
```

Вона має бути всередені `impl Solution`. На що тут варто звернути увагу?

По-перше, функція приймає `&str`. Це слайс (вказівник на діапазон значень всередені іншого обʼєкта) строки. Це невеличка оптимізація, яка дозволяє мати у памʼяті один екземпляр строки з регуляркою, і не створювати нових строк для кожного залишку патерна (і не копіювати зайвий раз).

По-друге, функція має лайфтайм, який використовується як у параметрі, так і у типі результата. Це означає, що значення, що поверне функція, буде мати всередені посилання на обʼєкт, переданий у функцію.

По-третє, функція повертає Option. Це алгебраїчний тип для (можливо) відсутнього значення. Ми повернемо `None`, коли дійдемо до кінця регулярки.

Тепер, напишемо сігнатуру-заготовку для нашого рекурсивного алгоритму.

```
fn match_recursive<'a>(s: &str, p: Option<Pattern<'a>>) -> bool {
    false
}
```

і викличемо її:

```
#[allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        return Self::match_recursive(s.as_str(), None);
    }

    fn match_recursive<'a>(s: &str, p: Option<Pattern<'a>>) -> bool {
        false
    }

    fn next_pattern<'a>(regex: &'a str) -> Option<Pattern<'a>> {
        let mut next_index = 1;
        let mut chars = regex.chars();

        let next_char = chars.next();
        if next_char.is_none() {
            return None;
        }

        let match_char = next_char.unwrap();

        let allow_multiple = chars.next().unwrap_or_default() == '*';
        if allow_multiple {
            next_index = next_index + 1;
        }

        Some(Pattern {
            match_char,
            allow_multiple,
            next_regex: &regex[next_index..],
        })
    }
}

pub struct Pattern<'a> {
    pub match_char: char,
    pub allow_multiple: bool,
    pub next_regex: &'a str,
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case001() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }
}

fn main() {
    println!("Hello, world!");
}

```

Поки що наше рішення нічого не робить, але ми підготовили код для реалізації.
Як на мене, це дуже важливо - мати щось, що можна запустити і протестити, навіть якщо воно не працює. Це дозволить швидко перевіряти різні варіанти кода і робити маленькі зміни, які просто оцінити і перевірити.

## Початок реалізації

Загалом, як я вже казав, алгоритм буде виглядати майже як посимвольне перевіряння вхідної строки за допомогою патерна.

Запишемо початок рекурсії

```
pub fn is_match(s: String, p: String) -> bool {
    let first_pattern = Self::next_pattern(p.as_str());
    return Self::match_recursive(s.as_str(), first_pattern);
}

fn match_recursive<'a>(s: &str, p: Option<Pattern<'a>>) -> bool {
    if let Some(pattern) = p {
        false
    } else {
        return s.len() == 0;
    }
}
```

Тут ми заодно реалізували перевірку, чи охопив патерн усю строку. Якщо регулярка скінчилася, а строка ні, то функція поверне `false`.

## Співставлення одного символа

Наступним кроком реалізуємо співставлення одного фіксованого символа. Це вже корисний код, який вирішує частину наших вимог.
Надалі приводитиму лише функцію `match_recursive`, бо зміни будуть лише у ній.

```
fn match_recursive<'a>(s: &str, p: Option<Pattern<'a>>) -> bool {
    if let Some(pattern) = p {
        let next_char = s.chars().next();

        if let Some(next) = next_char {
            if next == pattern.match_char {
                return Self::match_recursive(&s[1..], Self::next_pattern(pattern.next_regex));
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return s.len() == 0;
    }
}
```

Ми отримуємо наступний символ вхідної строки і співставляємо його з патерном.
Що ми тут бачимо?

По-перше, наступного символа може і не бути. Це значить, що ми добігли кінця строки, але в нас ще залишилися патерни для співставлення.
Тоді ми одразу повертаємо `false`. Це не зовсім вірно у загальному випадку, пуста строка - це цілком коректний інпут,
але для частини, яку ми реалізуємо - співставлення лише одного символа без квантифікаторів - це буде коректно.

Далі, по-друге, якщо символ є, але не співпадає - повертаємо `false`. Це вже не спрощення, так і має бути.

І, по-третє, якщо символ співпав, то викликаємо рекурсивно алгоритм для наступного патерна і залишка строки.

Також, додамо декілька тест-кейсів:

```
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn case001() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn case002() {
        assert_eq!(Solution::is_match("aa".to_string(), "aa".to_string()), true);
    }

    #[test]
    fn case003() {
        assert_eq!(Solution::is_match("ab".to_string(), "ab".to_string()), true);
    }

    #[test]
    fn case004() {
        assert_eq!(Solution::is_match("".to_string(), "".to_string()), true);
    }

    #[test]
    fn case005() {
        assert_eq!(Solution::is_match("a".to_string(), "".to_string()), false);
    }

    #[test]
    fn case006() {
        assert_eq!(Solution::is_match("".to_string(), "a".to_string()), false);
    }

    #[test]
    fn case007() {
        assert_eq!(Solution::is_match("b".to_string(), "a".to_string()), false);
    }

    #[test]
    fn case008() {
        assert_eq!(
            Solution::is_match("ab".to_string(), "ac".to_string()),
            false
        );
    }
}
```

Усі тести проходять.

Також тривіально додаємо підтримку для `.` (будь-який символ):

```
...
    if pattern.match_char == '.' || next == pattern.match_char {
...
```

І тест-кейси.

```
#[test]
fn case009() {
    assert_eq!(Solution::is_match("ab".to_string(), "a.".to_string()), true);
}

#[test]
fn case010() {
    assert_eq!(
        Solution::is_match("abcd".to_string(), "....".to_string()),
        true
    );
}

#[test]
fn case011() {
    assert_eq!(
        Solution::is_match("abcd".to_string(), "...".to_string()),
        false
    );
}
```

Проста частина позаду. Як бачимо, головне - підготувати код до ітеративних змін, забезпечити просте написання та виклик тестів, писати як позитивні, так і негативні тест-кейси, і не намагатися одразу писати 100% правильний код. Видалення і виправлення - це нормально, а у ітеративній розробці - майже обовʼязкове.
