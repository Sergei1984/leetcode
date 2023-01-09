У цій статті розглянемо вирішення задачі з літкоду.
Щоб не брати щось просте, візьмемо задачу з підрозділу hard [Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/) і вирішимо його на мові програмування Rust.

Відразу скажу, що моє рішення не є оптимальним по швидкості чи по памʼяті, і ціль статті - показати ітеративно підходи до вирішення складних задач.

Також дисклеймер - я не дивився інші рішення, ні до сабміту свого, ні після.
Зважаючи на досить скромні результати по швидкості, є якийсь більш оптимальний алгоритм, до якого я не додумався.

## Умови задачі і розгляд проблеми

> Given an input string s and a pattern p, implement regular expression matching with support for `.` and `*` where:
>
> - `.` - matches any single character.​​​​
> - `*` - matches zero or more of the preceding element.
>
>   The matching should cover the entire input string (not partial).

Тобто, нам треба реалізувати алгоритм співставлення строки зі спрощеним регулярним виразом.

Маємо такий синтаксис:

- `a` - співставляється з одним символом `a`. Може бути будь який символ, окрім `.` та `*`, які мають спеціальне значення. Зверніть увагу, що екранування піддтримувати не потрібно, цього нема в умовах і рішення проходить тести і без нього.
- `.` - співставляється з будь яким _одним_ символом. Символ обовʼязково має бути присутній, пуста строка не рахується за співпадіння.
- `*` - квантифікатор "нуль або більше". Вказує на те, що попередній символ може зустрічатися від нуля і більше.

Також, суттєва умова - вираз має співпадати з усією строкою, а не з префіксом. Тобто, регулярка `a***b` не співпадає зі строкою `abc`. У цьому є відмінність від звичайних регулярок, які співставляються з підстрокою.

Хоча задача може здатися на вигляд дуже простою (як і здалося мені с першого погляду), вона не даремно знаходиться у розділі hard, як ми далі і побачимо.

Цю задачу складно навіть просто вирішити, і ще складніше вирішити ефективно. Складність її для мене була у тому, що є досить багато випадків, які, природьно, не указані в умовах, і до яких я доходив у процесі вирішення. Також ми побачимо, що алгоритм, який може здатися досить простим і прямолінійним, не є таким.

## Підготовка до вирішення

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

Відразу скажу, що цю задачу можна вирішити за допомогою звичайних регулярних виразів. Наш спрощений синтаксис є підмножиною синтаксису звичайних регулярних виразів.

Тому ми співставити вираз зі строкою за допомогою вбудованих регулярних виразів, і перевірити, що співпадіння дорівнює вхідній строчці (щоб виконати вимогу на повне співпадіння).

Якщо вам попадеться ця задача на співбесіді, то можете це зауважити. Але вам має бути дуже потрібна робота, щоб вирішувати на співбесіді hard задачі з літкода.

Ми реалізуємо власний алгоритм.

По-перше, можемо побачити, що наш алгоритм рекурсивний. Тобто, ми беремо перший патерн з регулярного виразу, і співставляємо його з початком строки. Якщо вдалося співставити, викликаємо рекурсивно алгоритм для залишку регулярки і залишку вхідної строки. Синтаксис нашої регулярки досить простий, і ми можемо реалізувати алгоритм без look-back.

Створимо структуру для опису патерну. Тут, насправді, я трохи забігаю наперед, бо починав я вирішення без цієї структури, але я не мав ціллю показати повністю хід моїх думок від початку до кінця, а хотів показати, як ітеративно вирішувати складні задачі. Тому най буде:

```
pub struct Pattern<'a> {
    pub match_char: char,
    pub allow_multiple: bool,
    pub next_regex: &'a str,
}
```

Структура досить проста. `match_char` містить символ, з яким іде співставлення, `allow_multiple` - якщо після символа була зірочка `*`, і `next_regex` - це залишок регулярки.

Також оголошуємо лайфтайм `'a`. Його потрібно вказувати, якщо структура у собі містить нестатичні посилання (`&`). Цей лайфтайм підкаже компілятору, що структура не має пережити дані, на які є посилання всередені неї (тобто дані, на які ми посилаємося, не можна видалити раніше, ніж структуру).

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

Вона має бути всередені `impl Solution`.
Ми не будемо розглядати, як вона працює, у деталях, тому що вона досить проста, і не має безпосереднього відношення до задачі.

На що тут варто звернути увагу?

По-перше, функція приймає `&str`. Це слайс - вказівник на діапазон значень всередені строки або масива. Це невеличка оптимізація, яка дозволяє мати у памʼяті один екземпляр строки з регуляркою, і не створювати нових строк для кожного залишку патерна (і не копіювати зайвий раз).

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

Поки що наше рішення нічого не робить, але ми підготовили оточення для подальшої реалізації.
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

Тут ми заодно реалізували перевірку, чи охопив патерн усю строку. Якщо регулярка скінчилася, а строка ні, то строка не співпала з патерном, і ми повернемо `false`.

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

## Співставлення з повторюваннями

Нагадаю умови, якщо після символу (у тому числі і `.`) іде зірочка `*`, то це означає, що символ може зустрічатися _нуль або більше_ разів.
Саме ця умова є найскладнішою у цьому завданні, і саме з цією умовою повʼязана купа неочевидних тест-кейсів,
на які я витратив найбільше часу.

На перший погляд, усе досить просто. Спочатку додамо декілька тест-кейсів:

```
#[test]
fn case012_match_multiple() {
    assert_eq!(
        Solution::is_match("aaad".to_string(), "a*d".to_string()),
        true
    );
}

#[test]
fn case013_match_multiple_to_one() {
    assert_eq!(
        Solution::is_match("ad".to_string(), "a*d".to_string()),
        true
    );
}

#[test]
fn case014_match_mutiple_to_zero() {
    assert_eq!(Solution::is_match("d".to_string(), "a*d".to_string()), true);
}
```

Бачимо, що вони валяться, окрім `case013_match_multiple_to_one`, який, по суті, виконується тією логікою, що ми уже реалізували, а тому не валиться.

Додамо просту реалізацію:

```
fn match_recursive<'a>(s: &str, p: Option<Pattern<'a>>) -> bool {
    if let Some(pattern) = p {
        let next_char = s.chars().next();

        if pattern.allow_multiple {
            false
        } else {
            if let Some(next) = next_char {
                if pattern.match_char == '.' || next == pattern.match_char {
                    return Self::match_recursive(
                        &s[1..],
                        Self::next_pattern(pattern.next_regex),
                    );
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    } else {
        return s.len() == 0;
    }
}
```

Логіку для співставлення з одним символом ми вже написали і потестили, тому робимо `if`, і залишаємо попередній код у `else`.

Поки що відразу повертаємо `false`, тестимо, тепер усе вірно, і нереалізовані тести валяться.

Пишемо реалізацію:

```
if let Some(next) = next_char {
    if pattern.match_char == '.' || next == pattern.match_char {
        return Self::match_recursive(&s[1..], Some(pattern));
    } else {
        return Self::match_recursive(&s, Self::next_pattern(pattern.next_regex));
    }
} else {
    return Self::match_recursive(&s, Self::next_pattern(pattern.next_regex));
}
```

По-перше: пуста вхідна строка - цілком корректний інпут для регекса з зірочкою.
Якщо ми не можемо прочитати наступний символ строки, то викликаємо співставлення далі.

Цим ми обробимо патерни типу `abc*` на строках `ab`.

Додамо ці тест-кейси:

```
#[test]
fn case015_match_mutiple_at_the_end_string() {
    assert_eq!(
        Solution::is_match("ab".to_string(), "abc*".to_string()),
        true
    );
}

#[test]
fn case016_match_several_mutiple_at_the_end_string() {
    assert_eq!(
        Solution::is_match("ab".to_string(), "abc*d*e*".to_string()),
        true
    );
}
```

Все працює.

Здавалося б, готово, але давайте спочатку перевіримо, чи буде працювати такий патерн на початку строки:

```
#[test]
fn case017_match_several_at_the_start_of_string() {
    assert_eq!(
        Solution::is_match("bc".to_string(), "a*bc".to_string()),
        true
    );
}

#[test]
fn case018_match_multiple_several_at_the_start_of_string() {
    assert_eq!(
        Solution::is_match("de".to_string(), "a*b*c*de".to_string()),
        true
    );
}
```

Спробуємо засабмітити, і тут нас чекає засідка.

> ## Wrong answer
>
> Input: s = `"aaa"`
>
> Pattern: p = `"a*a"`
>
> Expected: `true`
>
> Output: `false`

У чому ж помилка?
Справа у тому, що наш алгоритм "жадібний".
Він співставляє патерн, доки він співставляється, максимальну кількість разів.
У нашому випадку це призводить до того, що перший же `a*` досягає кінця строки, і у патерні залишається `a`, яке співставляється з пустою строкою.

Як це записати у коді?

Це і є сама складна частина задачі. Я перепробував декілька ідей:

- Перевіряв наступний патерн, і якщо його символ такий самий, як і у попереднього, то пропускав, якщо попередній співставився хоча б з одним символом.

  Це валилося на співставленнях патернів типу `a*aa` зі строкою `aaaaa`, бо там символів більше, ніж один.
  Генералізація цього випадку для `n` символів після патерну з зірочкою теж не допомогла, бо є випадки типу патерн `a*b*c*abc` і строки `abcabc`.

- Думав перевіряти з початку, а потім ще з кінця, але це по суті те саме. Також, пізніше, знайшов тест-кейси, де це не спрацює, наприклад `.*abc.*` на строчці `aaaaaaaa`.

## Розвʼязання

Що точно спрацює? І як взагалі цей алгоритм повинен перевіряти, щоб покрити усі випадки?

Спочатку додамо декілька тест-кейсів:

```
#[test]
fn case019_match_all() {
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
}

#[test]
fn case020() {
    assert_eq!(
        Solution::is_match("abc1111111d".to_string(), "a.c.*d".to_string()),
        true
    );
}

#[test]
fn case021() {
    assert_eq!(
        Solution::is_match("aa".to_string(), "a.*a".to_string()),
        true
    );
}

#[test]
fn case022_leetcode_test_case() {
    assert_eq!(
        Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()),
        true
    );
}

#[test]
fn case023_non_greedy() {
    let result = Solution::is_match("ab".to_string(), ".*c".to_string());
    assert_eq!(result, false);
}

#[test]
fn case024_regex_longer() {
    let result = Solution::is_match("aaa".to_string(), "aaaa".to_string());
    assert_eq!(result, false);
}

#[test]
fn case025_stop_char_same_as_pattern() {
    let result = Solution::is_match("aaa".to_string(), "a*a".to_string());
    assert_eq!(result, true);
}

#[test]
fn case026_multiple_zero_patterns() {
    let result = Solution::is_match("aaa".to_string(), "ab*a*c*d*e*f*a".to_string());
    assert_eq!(result, true);
}

#[test]
fn case027_simplest_zero_pattern() {
    let result = Solution::is_match("aa".to_string(), "a*a".to_string());
    assert_eq!(result, true);
}

#[test]
fn case028_simplest_zero_pattern_interaction() {
    let result = Solution::is_match("aa".to_string(), "a*b*a".to_string());
    assert_eq!(result, true);
}

#[test]
fn case029_fucked_up_lookup() {
    let result = Solution::is_match("aaacacaab".to_string(), ".*ab".to_string());
    assert_eq!(result, true);
}

#[test]
fn case030_zero_or_more_at_the_end() {
    let result = Solution::is_match("a".to_string(), "ab*".to_string());
    assert_eq!(result, true);
}
```

Деякі з них вигадані, деякі я додавав з літкодівських тест-кейсів, які у мене валилися.

(Зрозуміло, що кожен літкодівський тест-кейс, який у вас не пройшов, треба додавати до своїх тест-кейсів.)

Так який же універсальний код зможе точно співставити будь-який патерн, най і не з найкращим перформансом?
Це, звісно, брутфорс.

Коли у нас у регексі є патерн з зірочкою, то ми не можемо знати точно, коли зупинитися. Ми можемо додати деякі еврістики, але у загальному випадку точно знати ми не можемо.

Тому, у нашому коді, коли ми перевіряємо патерн з зірочкою, і він співпадає, то нам потрібно перевірити, насправді, два випадки: перший - це залишок строки з тим же патерном, і другий - це залишок строки з наступним патерном. Бо навіть співпадаючий символ може бути, насправді, кінцем патерна.

Напишемо код:

```
if pattern.allow_multiple {
    if let Some(next) = next_char {
        if pattern.match_char == '.' || next == pattern.match_char {

            // Here we add
            let is_matching_if_end_of_greed =
                Self::match_recursive(&s, Self::next_pattern(pattern.next_regex));

            if is_matching_if_end_of_greed {
                return true;
            }
            // end of add

            return Self::match_recursive(&s[1..], Some(pattern));
        } else {
            return Self::match_recursive(&s, Self::next_pattern(pattern.next_regex));
        }
    } else {
        return Self::match_recursive(&s, Self::next_pattern(pattern.next_regex));
    }
}
...
```

Це код проходить усі тест-кейси літкода, але не є оптимальним. Воно бʼє лише 22.96% по швидкості, і 8.15% по памʼяті.

Спробуємо трохи оптимізувати.

Перше, що спадає на думку - обробляти ситуацію типу `a*b`, коли патерн з зірочкою не є `.` (будь-який символ), і наступний патерн є "один точний символ" (`b`), і символи у патернах різні. У цьому випадку ми знаємо точно, коли співпадіння повинно зупинитися.

Напишемо код:

```
fn match_recursive<'a>(s: &str, p: Option<Pattern<'a>>) -> bool {
    if let Some(pattern) = p {
        let next_char = s.chars().next();

        if pattern.allow_multiple {
            let maybe_next_pattern = Self::next_pattern(pattern.next_regex);

            if let Some(next) = next_char {
                if pattern.match_char == '.' || next == pattern.match_char {
                    if pattern.match_char == '.'
                        || maybe_next_pattern
                            .as_ref()
                            .map(|n| {
                                n.allow_multiple
                                    || n.match_char == '.'
                                    || n.match_char == pattern.match_char
                            })
                            .unwrap_or(false)
                    {
                        let is_matching_if_end_of_greed =
                            Self::match_recursive(&s, maybe_next_pattern);

                        if is_matching_if_end_of_greed {
                            return true;
                        }
                    }

                    return Self::match_recursive(&s[1..], Some(pattern));
                } else {
                    return Self::match_recursive(&s, maybe_next_pattern);
                }
            } else {
                return Self::match_recursive(&s, maybe_next_pattern);
            }
        } else {
            if let Some(next) = next_char {
                if pattern.match_char == '.' || next == pattern.match_char {
                    return Self::match_recursive(
                        &s[1..],
                        Self::next_pattern(pattern.next_regex),
                    );
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    } else {
        return s.len() == 0;
    }
}
```

Тепер бʼє 16.30% по швидкості, але 96.3% по памʼяті.

Не дуже радіємо, тому що використання памʼяті дуже різниться від запуска до запуска,
навіть, якщо сабмітити той самий код декілька разів, споживання памʼяті може різнитися від 10% до 90% від інших.
Схоже, що літкод не робить тести на навантаження, і на таких маленьких обʼємах памʼяті похибка більша, ніж саме споживання.

Інша оптимізація - це замість строки працювати зі слайсом (масивом) байт, за умовами у вхідній строці і у патерні лише англійскі літери.
Строки у Rust юникодні, і один символ може складатися з декількох байтів.
Ітератор `&str::chars()` виконує пошук мультибайтових символів, і працює повільніше, ніж ітератор по масиву байт,
який просто збільшує вказівник.

```
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let first_pattern = Self::next_pattern(p.as_bytes());
        return Self::match_recursive(s.as_bytes(), first_pattern);
    }

    fn match_recursive<'a>(s: &[u8], p: Option<Pattern<'a>>) -> bool {
        if let Some(pattern) = p {
            let next_char = s.iter().next();

            if pattern.allow_multiple {
                let maybe_next_pattern = Self::next_pattern(pattern.next_regex);

                if let Some(next) = next_char {
                    if pattern.match_char == b'.' || *next == pattern.match_char {
                        if pattern.match_char == b'.'
                            || maybe_next_pattern
                                .as_ref()
                                .map(|n| {
                                    n.allow_multiple
                                        || n.match_char == b'.'
                                        || n.match_char == pattern.match_char
                                })
                                .unwrap_or(false)
                        {
                            let is_matching_if_end_of_greed =
                                Self::match_recursive(&s, maybe_next_pattern);

                            if is_matching_if_end_of_greed {
                                return true;
                            }
                        }

                        return Self::match_recursive(&s[1..], Some(pattern));
                    } else {
                        return Self::match_recursive(&s, maybe_next_pattern);
                    }
                } else {
                    return Self::match_recursive(&s, maybe_next_pattern);
                }
            } else {
                if let Some(next) = next_char {
                    if pattern.match_char == b'.' || *next == pattern.match_char {
                        return Self::match_recursive(
                            &s[1..],
                            Self::next_pattern(pattern.next_regex),
                        );
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        } else {
            return s.len() == 0;
        }
    }

    #[inline]
    fn next_pattern<'a>(regex: &'a [u8]) -> Option<Pattern<'a>> {
        let mut next_index = 1;
        let mut chars = regex.iter();

        let next_char = chars.next();
        if next_char.is_none() {
            return None;
        }

        let match_char = next_char.unwrap();

        let allow_multiple = *(chars.next().unwrap_or(&b' ')) == b'*';
        if allow_multiple {
            next_index = next_index + 1;
        }

        Some(Pattern {
            match_char: *match_char,
            allow_multiple,
            next_regex: &regex[next_index..],
        })
    }
}

#[derive(Debug)]
pub struct Pattern<'a> {
    pub match_char: u8,
    pub allow_multiple: bool,
    pub next_regex: &'a [u8],
}
```

Тепер по швидкості додалося десь 10%, стало 25.19%.
Пропоную забити і зупинитися на цьому.

Я думаю, що навряд чи ми значно пришвидшимося, доки використовуємо рекурсивний алгоритм.
Його використання дозволило нам тривіально реалізувати брут-форс співставлення, з досить зрозумілим кодом.
Циклічна версія алгоритму виглядала б досить монструозно, як на мене.

Також не впевнений, що можна поміняти цей код так, щоб спрацювали tail recursion або tail call оптимізації
(та і взагалі далеко не факт, що літкод запускає з потрібними флагами).

Тому, скоріш за все, швидші реалізації просто додумалися до більш оптимального алгоритму.

Наш алгоритм проходить тести літкоду, але лише тому, що там є обмеження по довжині як вхідної строки (20 символів).
Використання його на довгих строках або довгих патернах призведе до "комбінаторного вибуху" рекурсивних перевірок.

У реальному алгоритмі, скоріш за все, патерн ділиться на частини, які складаються з поряд розташованих однотипних патернів.
Наприклад, у патерні `ab*c*de` ми могли б виділити такий набір патернів:

- `a` (фіксований патерн)
- `b*c*` (повторюваний патерн)
- `de` (фіксований патерн)

Сгрупуємо їх так, щоб у нас утворилися пари "повторюваний патерн + фіксований патерн" (кожен з них може бути відсутній).
Візьмемо довший патерн для більшої наглядності

`abc*d*efg*h*ijklmn*`

- None + `ab`
- `c*d*` + `ef`
- `g*h*` + `ijklm`
- `n*` + None

Тепер, робимо наступне: шукаємо у строчці усі входження фіксованого патерну (через аналог метода index of).
Далі перевіряємо підстроки до кожного входження на співпадіння з повторюваним патерном з пари.

Усі підстроки, які не співпали, ми відкидаємо.
А кожну з тих, які співпали, перевіряємо наступною парою патернів.

Якщо пара не містить повторюваного патерну, перевіряємо на співпадіння з початком строки (це може бути перша пара патернів).
Якщо пара не містить фіксованого патерну, перевіряємо залишок до кінця строки (буває наприкінці строки).

У нас все ще є комбінаторна перевірка, але значно ефективніша.

Стаття вийшла і так дуже довгою, якщо буде цікаво, можу зробити таку реалізацію.

## Висновки і закінчення

80% успіху вирішення задачі - це підготування собі оточення, у якому можна швидко писати і запускати тести, а також пробувати різні варіанти і відкочувати зміни (тому можна створити локальний гіт-репозиторій і комітити туди).

Щоб не витрачати час тестування на налаштування оточення, треба це зробити заздалегідь.
Сайти з тестами влаштовані досить схоже, тому навіть якщо ви не знаєте, яка буде структура, підготуйте таку, як у літкоді.
Адаптувати буде швидше, ніж зробити з нуля.

У випадку Rust усе робиться швидко, але для інших мов може зайняти 5-10 хвилин, які можуть виявитися далеко не зайвими у кінці.

Також дуже важливо мати змогу писати код у IDE, до якої ви звикли.
Варто поцікавитися у інтервʼюера, чи можна це робити.
Деякі компанії (а також деякі сайти з тестами) забороняють перемикати вкладинки браузера, та перемикатися на інші програми.
У цьому випадку можна запропонувати зробити запис свого екрану, як альтернативу.
Не всі на це погодяться, тому що хочуть обробляти результати тестування (напів)автоматично, і можуть одразу відкидати "читерів", хто перемикався.

Ну і, звісно, велике питання, чи варто взагалі витрачати час на тести для компанії, яка вимагає написання такого, досить важкого навіть у звичному оточенні, теста у браузері.

Може, це досить очевидно, але головне - це, по-перше, зрозуміти умови задачі, а по-друге - створити алгоритм. Написати код по готовому алгоритму значно простіше, ніж зразу писати в надії ствоити алгоритм на ходу.

Також одразу можна зробити функції та структури (а також написати до них тести), які точно підійдуть до будь-якого алгоритму. У нашому випадку це структура з патерном та функція парсингу наступного патерну зі строки.

Варто запитати у інтервʼюера, що для нього важливіше: перформанс чи чистий код.
Також запитайте, якої складності він очікує алгоритм, деякі завдання мають тривіальні рішення зі складністю `O(n^2)`, і набагато складніші з меншою складністю.
