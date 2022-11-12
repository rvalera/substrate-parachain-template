# DOCFabric Parachain - Substrate Cumulus Parachain

El proyecto se centra en construir la infraestructura necesaria para almacenar, indexar y recuperar informacion relativa a documentos no estructurados en la blockchain de Polkadot, usando una extension en el framework FRAME de Substrate. Se quiere plantear un caso de aplicacion para Blockchain fuera del tradicional escenario de intercambio de valor y finanzas descentraliza, algo que en la Web3 debe ser un componente clave en la interacción de los usuarios como lo son los documentos.

Para esto se construyo un modelo basado en gamificacion que permite la interaccion de usuarios entorno a dos abstracciones principales, Sitios que son entidades geolocalizadas en el cual se aglomeran un conjunto de usuarios y en los cuales se distribuyen documentos, un documento para el proyecto es un archivo + la metadata asociadad, en lo que respecta a la metadata, se refiere a descriptores al documento que se utilizan posteriormente para busqueda y que puede ser proporcionado por el usuario o por procesos automaticos implementados en la Blockchain.

Dentro del ecosistema de trabajo se propone crear una moneda virtual que permita el desarrollo de las operaciones de publicacion de documentos, conformacion de sitios, promocion de contenido prioritario. Asi mismo la interaccion de los participantes se recompensaran con puntos de actividad, que sera contralada en la misma blockchain, con ello es posible que puedan ser canjeados por la moneda del ecosistema con la cual permitira tener el balance necesario para desarrollar distintos tipos de actividad en la Parachain.

En lo que respecta al ambiente gamificado, como las interacciones de los usuarios sobre el contenido se vera recompensada por puntos de actividad, se podran construir esquemas de ranking, trofeos e insignias, asi mismo los puntos de actividad podran intercambiarse con activos digitales de la plataforma como avatars. 

A nivel de implementacion se trata de una Parachain que utiliza algunos de los Pallet proporcionados por FRAME de Substrate y otros especializados en el manejo de contenido, en principio para este alcance se propuso la construccion de un Pallet que permite el almacenamiento y descarga de documentos y las operaciones de actividad sobre las entidades principales que maneja la plataforma.

El proyecto incluye algunos elementos de gamificacion, como la recompensa por visitar contenido y sitios, estos puntos de recompensa pueden ser intercambiados por monedas en el ecosistema que permite la realizacion de distintas operaciones.

![image](https://user-images.githubusercontent.com/1779865/201475879-5d4fe59b-7b55-4290-91af-cdc6b687664c.png)

Run Validators (in Polkadot base folder):

	./target/release/polkadot --alice --validator --base-path ../tmp/relay/alice --chain ../tmp/raw-local-chainspec.json --port 30333 --ws-port 9944
	./target/release/polkadot --bob --validator --base-path ../tmp/relay-bob --chain ../tmp/raw-local-chainspec.json --port 30334 --ws-port 9945
	//Check Validators of Parachain
	https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/chainstate

Build Parachain (in Parachain base folder):

	cargo build --release

Configure Parachain (in Parachain base folder):

	./target/release/parachain-template-node build-spec --disable-default-bootnode > plain-parachain-chainspec.json
	// EDIT The File and Add the required change to assign Slot to Parachain 
	./target/release/parachain-template-node build-spec --chain plain-parachain-chainspec.json --disable-default-bootnode --raw > raw-parachain-chainspec.json
	./target/release/parachain-template-node export-genesis-wasm --chain raw-parachain-chainspec.json para-2000-wasm
	./target/release/parachain-template-node export-genesis-state --chain raw-parachain-chainspec.json para-2000-genesis-state

Run Collator (in Parachain base folder):

	./target/release/parachain-template-node --alice --collator --force-authoring --chain raw-parachain-chainspec.json --base-path ../tmp/parachain/alice --port 40333 --ws-port 8844 -- --execution wasm --chain ../tmp/raw-local-chainspec.json --port 30343 --ws-port 9977
	// Check Status of Blockchain of Parachain
	https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A8844#/chainstate	

Extrinsics Created in Parachain

![image](https://user-images.githubusercontent.com/1779865/201477153-7693cc7f-8fd5-40c7-a095-783fe0fb940f.png)

Query data in Parachain 

![image](https://user-images.githubusercontent.com/1779865/201477178-764e3739-9dc9-439d-9920-cd73e75d2376.png)


