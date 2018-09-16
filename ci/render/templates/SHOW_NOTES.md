# Episode {{ episode.number }} - {{ episode.title | title }}

[![YouTube video thumbnail](./thumb.jpg)](https://hello-rust.show/{{ episode.number }}/)
**[&#x25b6; Watch now on Youtube!](https://youtu.be/{{episode.id}})**

{{ episode.intro }}  
{%- if episode.details %}
{{ episode.details }}  
{%- endif %}


If you like to get notified about new episodes, [please subscribe to my channel](https://www.youtube.com/hellorust) 😊.

Keywords: {{ episode.keywords | join(sep=", ") }}

## Things I mentioned during the show

{% for note in episode.notes -%}
* {{ note }}
{% endfor %}

{%- if episode.others %}
## Things I should have mentioned (but forgot)

{% for other in episode.others -%}
* {{ other }}
{% endfor %}
{% endif -%}

{%- if episode.errata %}
## Errata and improvements

It might come as a surprise to you, but every once in a while *even I* make a mistake.  
This section covers all improvements made to the code since the epsiode went live.  
For an exhaustive list of all changes to the original code, [go here](https://github.com/hello-rust/show/commits/master/episode/{{ episode.number }}).
Thanks to all contributors!  

{% for error in episode.errata -%}
* {{ error }}
{% endfor -%}
{%- endif -%}

{% if episode.metas %}
## Meta

{% for meta in episode.metas -%}
* {{ meta }}
{% endfor %}
{% endif %}

{%- if episode.licenses %}
## Resources and licenses

{% for license in episode.licenses -%}
* {{ license }}
{% endfor %}
{% endif %}

## Support!

Preparing, recording, and editing an episode takes a substantial amount of time
(around 30 hours total). I do all of this next to my fulltime dayjob.
If you want to show your appreciation and help me keep the content free
for everybody to enjoy, [please consider supporting me on
Patreon](https://www.patreon.com/bePatron?c=1568097) - no matter the amount. ❤️