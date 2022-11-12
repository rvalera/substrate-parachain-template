# DOCFabric Parachain - Substrate Cumulus Parachain

El proyecto se centra en construir la infraestructura necesaria para almacenar, indexar y recuperar informacion relativa a documentos no estructurados en la blockchain de Polkadot, usando una extension en el framework FRAME de Substrate. Se quiere plantear un caso de aplicacion para Blockchain fuera del tradicional escenario de intercambio de valor y finanzas descentraliza, algo que en la Web3 debe ser un componente clave en la interacción de los usuarios como lo son los documentos.

Para esto se construyo un modelo basado en gamificacion que permite la interaccion de usuarios entorno a dos abstracciones principales, Sitios que son entidades geolocalizadas en el cual se aglomeran un conjunto de usuarios y en los cuales se distribuyen documentos, un documento para el proyecto es un archivo + la metadata asociadad, en lo que respecta a la metadata, se refiere a descriptores al documento que se utilizan posteriormente para busqueda y que puede ser proporcionado por el usuario o por procesos automaticos implementados en la Blockchain.

Dentro del ecosistema de trabajo se propone crear una moneda virtual que permita el desarrollo de las operaciones de publicacion de documentos, conformacion de sitios, promocion de contenido prioritario. Asi mismo la interaccion de los participantes se recompensaran con puntos de actividad, que sera contralada en la misma blockchain, con ello es posible que puedan ser canjeados por la moneda del ecosistema con la cual permitira tener el balance necesario para desarrollar distintos tipos de actividad en la Parachain.

En lo que respecta al ambiente gamificado, como las interacciones de los usuarios sobre el contenido se vera recompensada por puntos de actividad, se podran construir esquemas de ranking, trofeos e insignias, asi mismo los puntos de actividad podran intercambiarse con activos digitales de la plataforma como avatars. 

A nivel de implementacion se trata de una Parachain que utiliza algunos de los Pallet proporcionados por FRAME de Substrate y otros especializados en el manejo de contenido, en principio para este alcance se propuso la construccion de un Pallet que permite el almacenamiento y descarga de documentos y las operaciones de actividad sobre las entidades principales que maneja la plataforma.

El proyecto incluye algunos elementos de gamificacion, como la recompensa por visitar contenido y sitios, estos puntos de recompensa pueden ser intercambiados por monedas en el ecosistema que permite la realizacion de distintas operaciones.
