use lazy_static::lazy_static;

macro_rules! valid_attributes {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        lazy_static!{
        static ref VALID_ATTRIBUTES:Vec<&'static str> =
            vec![
                $(
                  stringify!($name)
                 ),*
            ];
        }
    }
}

macro_rules! keyword_attributes {
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        lazy_static!{
        static ref KEYWORD_ATTRIBUTES: Vec<(&'static str, &'static str)> =
            vec![
                $(
                    (stringify!($name), $attribute)
                 ),*
            ];
        }
    }
}
valid_attributes! {
    accept;
    accesskey;
    action;
    align;
    allow;
    alt;
    autocapitalize;
    autocomplete;
    autofocus;
    autoplay;
    background;
    bgcolor;
    border;
    buffered;
    challenge;
    charset;
    checked;
    cite;
    class;
    code;
    codebase;
    color;
    cols;
    colspan;
    content;
    contenteditable;
    contextmenu;
    controls;
    coords;
    crossorigin;
    csp;
    data;
    datetime;
    decoding;
    default;
    defer;
    dir;
    dirname;
    disabled;
    download;
    draggable;
    dropzone;
    enctype;
    enterkeyhint;
    formaction;
    formnovalidate;
    headers;
    height;
    hidden;
    high;
    href;
    hreflang;
    http;
    icon;
    id;
    importance;
    integrity;
    intrinsicsize;
    inputmode;
    ismap;
    itemprop;
    keytype;
    kind;
    lang;
    language;
    loading;
    list;
    low;
    manifest;
    max;
    maxlength;
    minlength;
    media;
    method;
    min;
    multiple;
    muted;
    name;
    novalidate;
    open;
    optimum;
    pattern;
    ping;
    placeholder;
    poster;
    preload;
    radiogroup;
    readonly;
    referrerpolicy;
    rel;
    required;
    reversed;
    rows;
    rowspan;
    sandbox;
    scope;
    scoped;
    selected;
    shape;
    size;
    sizes;
    slot;
    spellcheck;
    src;
    srcdoc;
    srclang;
    srcset;
    start;
    step;
    style;
    summary;
    tabindex;
    target;
    title;
    translate;
    usemap;
    value;
    width;
    wrap;
    key;
}

// attributes with dash
keyword_attributes! {
    r#loop => "loop";
    r#type => "type";
}

pub fn format(k: &str, v: &str) -> String {
    if VALID_ATTRIBUTES.contains(&k) {
        //TODO also convert value to integer or boolean if they are
        format!(r#"{}("{}")"#, k, v)
    } else if KEYWORD_ATTRIBUTES
        .iter()
        .find(|(_ident, att)| *att == k)
        .is_some()
    {
        format!(r#"r#{}("{}")"#, k, v)
    } else {
        format!(r#"attr("{}", "{}")"#, k, v)
    }
}

pub fn is_valid(k: &str) -> bool {
    if VALID_ATTRIBUTES.contains(&k) {
        true
    } else if KEYWORD_ATTRIBUTES
        .iter()
        .find(|(_ident, att)| *att == k)
        .is_some()
    {
        true
    } else {
        false
    }
}
