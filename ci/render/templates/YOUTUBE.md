✅ Subscribe: https://www.youtube.com/c/hellorust
💖 Sponsor: https://github.com/sponsors/mre
🏠 Website: https://hello-rust.show
👨‍💻️ Github: https://github.com/hello-rust/show  
🐦 Twitter: https://twitter.com/hellorustshow  

{{ episode.intro }} 🤓 

{% if episode.chapters-%}
Chapters
{% for chapter in episode.chapters -%}
{{ chapter }}
{% endfor %}
{% endif -%}

Important Links:
{% for note in episode.notes -%}
* {{ note }}
{% endfor %}
Missing something? Find even more information in the show notes: 
https://github.com/hello-rust/show/tree/master/episode/{{ episode.number }}

{% if episode.licenses -%}
Licensing
{% for license in episode.licenses -%}
{{ license }}
{% endfor %}
{% endif -%}

Keywords: {{ episode.keywords | join(sep=", ") }}

Hello Rust! is a show about the Rust programming language.  
My goal is to make systems programming accessible and entertaining for everyone.
