# Episode {{ episode.number }} - {{ episode.title | title }}

![YouTube video thumbnail](./thumb.jpg)

{{ episode.intro }}  
{%- if episode.details %}
{{ episode.details }}  
{%- endif %}

[Watch now on Youtube!]({{ episode.url }})  

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
## Errata

It might come to you as a surprise, but every once in a while even I make a mistake.  
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

If you liked this video, [please subscribe to my channel](https://www.youtube.com/channel/UCZ_EWaQZCZuGGfnuqUoHujw) 😊.
You will not regret this, I guess.
If you're really digging this thing, [show your support on Patreon](https://www.patreon.com/bePatron?c=1568097).  
Thank you very much for considering a donation - no matter the amount.
