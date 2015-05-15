var elems = document.querySelectorAll('body > div');
var banner = elems[0];
var name_ = elems[1];
var date_and_sha = elems[2];
var desc = elems[3];
var diff = elems[4];

function fade_in_at(s, el) {
    window.setTimeout(function() {
	el.setAttribute('style', 'opacity: 1');
    }, s * 1000);
}
fade_in_at(2, banner);
fade_in_at(6, name_);
fade_in_at(14, date_and_sha);
fade_in_at(14, desc);
fade_in_at(14, diff);
