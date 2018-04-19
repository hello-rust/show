# Episode {{ episode.number }} - {{ episode.title | title }}

{{ episode.intro }}  
{{ episode.details }}  

[Watch now on Youtube!]({{ episode.url }})  

Keywords: {{ episode.keywords | join(sep=", ") }}

## Things I mentioned during the show

{% for note in episode.notes -%}
* {{ note }}
{% endfor %}

## Things I should have mentioned (but forgot)

{% for other in episode.others -%}
* {{ other }}
{% endfor %}

## Meta

{% for meta in episode.metas -%}
* {{ meta }}
{% endfor %}

## Support!

If you liked this video, [please subscribe to my channel](https://www.youtube.com/channel/UCZ_EWaQZCZuGGfnuqUoHujw).  
You will not regret this, I guess.
If you're really digging this thing, [show your support on Patreon](https://www.patreon.com/hellorust).
