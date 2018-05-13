{{ episode.intro }} 🤓 

{% if episode.markers -%}
Markers (for navigation):

{% for marker in episode.markers -%}
{{ marker }}
{% endfor %}
{% endif -%}

Important Links:
{% for note in episode.notes -%}
* {{ note }}
{% endfor %}
Missing something? Find even more information in the show notes: https://github.com/hello-rust/show/episode/{{ episode.number }}

Keywords: {{ episode.keywords | join(sep=", ") }}

Hello Rust! is a show about the Rust programming language.
My goal is to address beginner and intermediate Rust questions and show that systems programming can be a lot of fun!

Please subscribe: https://www.youtube.com/channel/UCZ_EWaQZCZuGGfnuqUoHujw 😊 

WEBSITE: http://hello-rust.show/  
GITHUB: https://github.com/hello-rust/show  
TWITTER: https://twitter.com/hellorustshow  

💖 Support the show by becoming a patron at https://www.patreon.com/hellorust