#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	// use frame_support::{pallet_prelude::*, inherent::Vec, sp_runtime::traits::Hash, transactional, };	
	// use frame_system::pallet_prelude::*;

	use frame_support::{
		ensure,
		inherent::Vec,
		pallet_prelude::*,
		sp_runtime::traits::{AccountIdConversion, Saturating, Zero, Hash},
		storage::child,
		traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons},
		PalletId,
		transactional,
	};

	use frame_system::{pallet_prelude::*,ensure_signed};

	use sp_runtime::traits::AtLeast32BitUnsigned;

	use super::*;	

	const PALLET_ID: PalletId = PalletId(*b"docfabri");	

	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
		
	// use scal{e::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use serde_json_core::from_slice;	

    #[derive(Deserialize, Serialize, Encode, Debug, PartialEq, TypeInfo)]
    pub struct MetadataRaw <'a> {
        pub title: &'a str,
        pub description: &'a str,
        pub author: &'a str,
		pub keywords : &'a str,
		pub content_type_id : &'a str,
		pub site_id : &'a str,
		pub file_name : &'a str,
		pub created_time : &'a str,
		pub update_time : &'a str,
		// Define a set of attributes to store in the
    }	

    #[derive(Deserialize, Serialize, Encode, Debug, PartialEq, TypeInfo)]
    pub struct ContentTypeRaw <'a> {
		pub name: &'a str,
        pub description: &'a str,
		//pub fields : Vec<FieldTypeRaw <'a>>,
    }	

    #[derive(Deserialize, Serialize, Encode, Debug, PartialEq, TypeInfo)]
    pub struct FieldTypeRaw <'a> {
		pub name: &'a str,
        pub description: &'a str,
		pub primitive_type : &'a str,
		// Define a set of attributes to store in the
    }	

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Site<T: Config> {
		pub id : T::Hash,
		pub name: Vec<u8>,
		pub description: Vec<u8>,
		pub owner: T::AccountId,
		pub affiliated : Vec<T::AccountId>,
		pub followers : Vec<T::AccountId>,
		pub lovers : Vec<T::AccountId>,
		pub promoted : Vec<T::Hash>,
	}	

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct PartnerImpression<T: Config> {
		pub site_id : T::Hash,
		pub partner: T::AccountId,
		pub payed: u16,
		pub showed: u16,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Content<T: Config> {
		pub id : T::Hash,
		pub content: Vec<u8>,
		pub site_id : T::Hash,
		pub content_type : T::Hash,
		pub metadata :  T::Hash,
		pub owner: <T as frame_system::Config>::AccountId,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct ContentMetadata<T: Config> {
		pub id : T::Hash,		
        pub title: Vec<u8>,
        pub description: Vec<u8>,
        pub author: Vec<u8>,
		pub keywords : Vec<u8>,
		pub file_name : Vec<u8>,
		pub created_time : Vec<u8>,
		pub update_time : Vec<u8>,
		pub binary_data : T::Hash,		
		pub content_type : T::Hash,
		pub site_id : T::Hash,
		pub owner: <T as frame_system::Config>::AccountId,
		// pub fields: Vec<FieldValue>
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct FieldValue {
		pub name: Vec<u8>,
        pub value: Vec<u8>,
	}	

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct ContentType<T: Config> {
		pub id : T::Hash,		
		// pub site_id : T::Hash,
		pub name: Vec<u8>,
        pub description: Vec<u8>,
		//pub fields : Vec<FieldType>, 
		pub owner: <T as frame_system::Config>::AccountId
	}	

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct FieldType {
		pub name: Vec<u8>,
        pub description: Vec<u8>,
        pub primitive_type : Vec<u8>,
	}	

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        UserEntryCreated(Vec<u8>, T::AccountId, T::Hash),
        ContentTypeCreated(Vec<u8>, T::AccountId, T::Hash),
        GetUserEntry(Vec<u8>, T::AccountId, T::Hash),
        SiteCreated(Vec<u8>, T::AccountId, T::Hash),
		SiteAffiliated(T::Hash, Vec<u8>,  T::AccountId,  T::AccountId, BalanceOf<T>),
		SiteFollowed(T::Hash, Vec<u8>,  T::AccountId,  T::AccountId),
		SiteLoved(T::Hash, Vec<u8>,  T::AccountId,  T::AccountId),
		RewardEarned(T::AccountId,T::Balance),
		ImpressionPayed(T::Hash, T::AccountId, BalanceOf<T>, u16),
		ImpressionViewed(T::Hash,  T::AccountId,  T::AccountId),
		ContentPromoted(T::Hash, T::Hash, T::AccountId),
	}	

	#[pallet::error]
	pub enum Error<T> {
		/// The requested user has not stored a value yet
		NoValueStored,
		/// The value cannot be incremented further because it has reached the maximum allowed value
		MaxValueReached,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		// type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Call: From<Call<Self>> + IsType<<Self as frame_system::Config>::Call>;

		/// The currency trait.
		type Currency: Currency<Self::AccountId>;

		// The type used to store balances.
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;		
	}

	#[pallet::storage]
	#[pallet::getter(fn sites)]
	pub(super) type Sites<T: Config> = StorageMap<	_, 
		Blake2_128Concat, 
		T::Hash, 
		Site<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn all_sites)]
	pub(super) type AllSites<T: Config> = StorageValue<_, Vec<Site<T>>, ValueQuery>;	

	// #[pallet::storage]
	// #[pallet::getter(fn site_affiliations)]
	// pub(super) type SiteAfilliations<T: Config> = StorageMap<	_, 
	// 	Blake2_128Concat, 
	// 	T::Hash, 
	// 	Vec<T::AccountId>,
	// 	OptionQuery,
	// >;	

	// #[pallet::storage]
	// #[pallet::getter(fn site_followers)]
	// pub(super) type SiteFollowers<T: Config> = StorageMap<_, 
	// 	Blake2_128Concat, 
	// 	T::Hash, 
	// 	Vec<T::AccountId>,
	// 	OptionQuery,
	// >;	

	// #[pallet::storage]
	// #[pallet::getter(fn site_lovers)]
	// pub(super) type SiteLovers<T: Config> = StorageMap<_, 
	// 	Blake2_128Concat, 
	// 	T::Hash, 
	// 	Vec<T::AccountId>,
	// 	OptionQuery,
	// >;	

	#[pallet::storage]
	#[pallet::getter(fn site_impressions)]
	pub(super) type SiteImpressions<T: Config> = StorageDoubleMap<_, 
		Blake2_128Concat, 
		T::Hash, 
		Blake2_128Concat,
		T::AccountId,
		PartnerImpression<T>,
		OptionQuery,
	>;	

	// #[pallet::storage]
	// #[pallet::getter(fn promotionated_content)]
	// pub(super) type PromotionatedContent<T: Config> = StorageMap< _, 
	// 	Blake2_128Concat, 
	// 	T::Hash,  
	// 	Vec<ContentMetadata<T>>,
	// 	OptionQuery 
	// >; 

	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type BalanceToAccount<T: Config> = StorageMap< _, 
		Blake2_128Concat, 
		T::AccountId, 
		T::Balance,
		ValueQuery
		>;	

	#[pallet::storage]
	#[pallet::getter(fn all_metadata)]
	pub(super) type Metadata<T: Config> = StorageMap<_, 
		Twox64Concat, 
		T::Hash, 
		ContentMetadata<T>
		>;

	#[pallet::storage]
	#[pallet::getter(fn all_content)]
	pub(super) type Binaries<T: Config> = StorageMap<_, 
		Twox64Concat, 
		T::Hash, 
		Content<T>
		>;

	#[pallet::storage]
	#[pallet::getter(fn user_entries)]
	pub(super) type UserEntries<T: Config> = StorageMap<_, 
		Twox64Concat, 
		T::AccountId, 
		Vec<ContentMetadata<T>> , 
		ValueQuery
		>;

	#[pallet::storage]
	#[pallet::getter(fn site_entries)]
	pub(super) type SiteEntries<T: Config> = StorageMap<_, 
		Twox64Concat, 
		T::Hash,  
		Vec<ContentMetadata<T>> , 
		ValueQuery
		>;		

	#[pallet::storage]
	#[pallet::getter(fn all_entries)]
	pub(super) type AllEntries<T: Config> = StorageValue<_, Vec<ContentMetadata<T>>, ValueQuery>;	
	

	#[pallet::storage]
	#[pallet::getter(fn all_content_types)]
	pub(super) type AllContenTypes<T: Config> = StorageValue<_, Vec<ContentType<T>>, ValueQuery>;	

	#[pallet::storage]
	#[pallet::getter(fn content_types)]
	pub(super) type ContenTypes<T: Config> = StorageMap<_, 
		Twox64Concat, 
		T::Hash, 
		ContentType<T> >;

	#[pallet::storage]
	#[pallet::getter(fn user_content_types)]
	pub(super) type UserContenTypes<T: Config> = StorageMap<_, 
		Twox64Concat, 
		T::AccountId, 
		Vec<ContentType<T>>, 
		ValueQuery
		>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}	

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10000)]
		// #[transactional]
		pub fn create_user_entry(origin: OriginFor<T>, 
			site_id : T::Hash,
			content_type_id : T::Hash,			 
			metadata: Vec<u8>, 
			binary: Vec<u8> ) -> DispatchResult {

			let owner = ensure_signed(origin)?;

			let (metadata_raw, _): (MetadataRaw, usize) = serde_json_core::from_slice(&metadata).unwrap();
			let user_entry_metadata_id = T::Hashing::hash_of(&metadata);
			let user_entry_binary_id = T::Hashing::hash_of(&binary);

			// Validar que el tipo de Contenido exista y que el propietario sea el definido
			let content_type = ContenTypes::<T>::get(&content_type_id);

			let new_user_medatata = ContentMetadata {
				id : user_entry_metadata_id,		
				title: Vec::from(metadata_raw.title),
				description: Vec::from(metadata_raw.description),
				author: Vec::from(metadata_raw.author),
				keywords : Vec::from(metadata_raw.keywords),
				file_name : Vec::from(metadata_raw.file_name),
				created_time : Vec::from(metadata_raw.created_time),
				update_time : Vec::from(metadata_raw.update_time),
				owner: owner.clone(),
				binary_data : user_entry_binary_id.clone(),
				content_type : content_type_id.clone(),
				site_id : site_id.clone()
			};

			let user_content = Content { 
				content: binary, 
				content_type : content_type_id.clone(),
				id : user_entry_binary_id, 
				site_id : site_id.clone(),
				metadata : user_entry_metadata_id.clone(),
				owner: owner.clone() 
			};

			<Metadata<T>>::insert(user_entry_metadata_id.clone(), new_user_medatata.clone());
			<Binaries<T>>::insert(user_entry_binary_id.clone(), user_content);

			let mut user_entries = UserEntries::<T>::get(&owner);
			//user_entries.push(user_entry_metadata_id.clone());
			user_entries.push(new_user_medatata.clone());
			UserEntries::<T>::insert(owner.clone(),user_entries);

			let mut site_entries = SiteEntries::<T>::get(&site_id);
			//user_entries.push(user_entry_metadata_id.clone());
			site_entries.push(new_user_medatata.clone());
			SiteEntries::<T>::insert(site_id.clone(),site_entries);		
			
			let mut all_content   = AllEntries::<T>::get();
			all_content.push(new_user_medatata.clone());
			AllEntries::<T>::put(all_content);				

			Self::deposit_event(Event::UserEntryCreated(metadata, owner, user_entry_metadata_id));

			Ok(())
		}

		#[pallet::weight(10000)]
		pub fn create_content_type(origin: OriginFor<T>,
			// site_id : T::Hash, 
			conten_type_raw: Vec<u8> ) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let (content_type, _): (ContentTypeRaw, usize) = serde_json_core::from_slice(&conten_type_raw).unwrap();

			let content_type_id = T::Hashing::hash_of(&conten_type_raw);
			let new_content_type  = ContentType {
				id : content_type_id.clone(),
				// site_id : site_id.clone(),
				name : Vec::from(content_type.name),
				description: Vec::from(content_type.description),
				// fields: Vec::from(content_type.fields),
				// fields: Vec::<u8>::new(),
				owner: owner.clone() 
			};

			let mut user_definitions = UserContenTypes::<T>::get(&owner);
			user_definitions.push(new_content_type.clone());
			UserContenTypes::<T>::insert(owner.clone(),user_definitions);

			ContenTypes::<T>::insert(content_type_id.clone(),new_content_type.clone());

			let mut all_content_types  = AllContenTypes::<T>::get();
			all_content_types.push(new_content_type.clone());
			AllContenTypes::<T>::put(all_content_types);

			Self::deposit_event(Event::ContentTypeCreated(conten_type_raw, owner, content_type_id));

			Ok(())
		}

		#[pallet::weight(10000)]
		pub fn create_site(origin: OriginFor<T>, 
			name: Vec<u8>, 
			description: Vec<u8> ) -> DispatchResult {

			let owner = ensure_signed(origin)?;

			let site_id = T::Hashing::hash_of(&(name.clone(),description.clone()) );

			let new_site = Site {
				id : site_id,
				name : name.clone(),
				description : description,
				owner : owner.clone(),
				followers : Vec::new(),
				affiliated : Vec::new(),
				lovers : Vec::new(),
				promoted : Vec::new(),
			};

			Sites::<T>::insert(site_id,new_site.clone());
			let mut all_sites = AllSites::<T>::get();
			all_sites.push(new_site.clone());
			AllSites::<T>::put(all_sites);

			Self::deposit_event(Event::SiteCreated(name.clone(), owner.clone(), site_id.clone()));

			Ok(())
		}

		#[pallet::weight(10000)]
		pub fn make_affiliation(origin: OriginFor<T>, 
			site_id : T::Hash ) -> DispatchResult {
			let who = ensure_signed(origin)?;			
			
			let mut site = Sites::<T>::get(&site_id).unwrap();

			let value : BalanceOf<T> = 100u32.into();

			site.affiliated.push(who.clone());
			Sites::<T>::insert(site_id.clone(),site.clone());

			// let mut afilliations = SiteAfilliations::<T>::get(&site_id).unwrap();
			// afilliations.push(who.clone());
			// SiteAfilliations::<T>::insert(site_id.clone(),afilliations);
			
			T::Currency::transfer(
				&who,
				&site.owner,
				value,
				ExistenceRequirement::AllowDeath
			)?;

			Self::deposit_event(Event::SiteAffiliated(site_id.clone(), site.name.clone(), site.owner.clone(),  who.clone(), value.clone()));
	
			Ok(())
		}

		//TODO: Para pagar impresiones en un sitio web primero debe estar afiliado al sitio web
		#[pallet::weight(10000)]
		pub fn pay_impression(origin: OriginFor<T>, 
			site_id : T::Hash, 
			amount : u16 ) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let site = Sites::<T>::get(&site_id).unwrap();

			//TODO: Necesitamos saber el usuario esta afiliado al sitio, antes de pagar las impresiones

			//TODO: Crear un DoubleMap para almacenar la informacion de impresiones, Sitio x Usuario
			let partner_impression = PartnerImpression {
				site_id : site_id.clone(),
				partner : who.clone(),
				payed : amount,
				showed : 0
			};
			SiteImpressions::<T>::insert(&site_id,&who,partner_impression);

			let value : BalanceOf<T>  = amount.into();

			T::Currency::transfer(
				&who,
				&site.owner,
				value,
				ExistenceRequirement::AllowDeath
			)?;

			Self::deposit_event(Event::ImpressionPayed(site_id.clone(), who.clone(), value.clone(),amount));
	
			Ok(())
		}		

		#[pallet::weight(10000)]
		pub fn view_impression(origin: OriginFor<T>,
			site_id : T::Hash, 
			content_id : T::Hash) -> DispatchResult {
			let site = Sites::<T>::get(&site_id).unwrap();
			let who = ensure_signed(origin)?;
			let metadata_content = Metadata::<T>::get(&content_id).unwrap();
			let owner_content = metadata_content.owner;
			
			//TODO: Crear un DoubleMap para actualizar la informacion de impresiones, Sitio x Usuario
			if (SiteImpressions::<T>::contains_key(&site_id,&owner_content)) {
				let mut partner_impression = SiteImpressions::<T>::get(&site_id,&owner_content).unwrap();

				let showed = partner_impression.showed;
				partner_impression.showed = showed + 1;
				SiteImpressions::<T>::insert(&site_id,&owner_content,partner_impression.clone());				
			}

			//Recompensa al usuario que visito el contenido promocionado
			let amount: T::Balance = 50u8.into();

			if Self::increase_balance(&who, amount) {
				Self::deposit_event(Event::<T>::RewardEarned(who.clone(),amount.clone()));
			}	
			
			Self::deposit_event(Event::<T>::ImpressionViewed(content_id.clone(),who.clone(),owner_content.clone()));

			Ok(())
		}

		#[pallet::weight(10000)]
		pub fn follow_site(origin: OriginFor<T>, 
			site_id : T::Hash ) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut site = Sites::<T>::get(&site_id).unwrap();

			site.followers.push(who.clone());
			Sites::<T>::insert(site_id.clone(),site.clone());

			// let mut followers = SiteFollowers::<T>::get(&site_id).unwrap();
			// followers.push(who.clone());
			// SiteFollowers::<T>::insert(site_id.clone(),followers);

			let amount: T::Balance = 200u8.into();

			if Self::increase_balance(&who, amount) {
				Self::deposit_event(Event::<T>::RewardEarned(who.clone(),amount.clone()));
			}			

			Self::deposit_event(Event::SiteFollowed(site_id.clone(), site.name.clone(), site.owner.clone(),  who.clone() ));

			Ok(())
		}

		#[pallet::weight(10000)]
		pub fn love_site(origin: OriginFor<T>, 
			site_id : T::Hash ) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut site = Sites::<T>::get(&site_id).unwrap();

			site.lovers.push(who.clone());
			Sites::<T>::insert(site_id.clone(),site.clone());

			// let mut lovers = SiteLovers::<T>::get(&site_id).unwrap();
			// lovers.push(who.clone());
			// SiteLovers::<T>::insert(site_id.clone(),lovers);


			let amount: T::Balance = 250u8.into();

			if Self::increase_balance(&who, amount) {
				Self::deposit_event(Event::<T>::RewardEarned(who.clone(),amount.clone()));
			}			

			Self::deposit_event(Event::SiteLoved(site_id.clone(), site.name.clone(), site.owner.clone(),  who.clone() ));

			Ok(())
		}			


		//TODO: Para promover contenido debe primero estar afiliado y ademas tener contratado impresiones en el sitio
		#[pallet::weight(10000)]
		pub fn promote_content(origin: OriginFor<T>, 
			site_id : T::Hash, 
			content_id : T::Hash ) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut site = Sites::<T>::get(&site_id).unwrap();
			let content = Metadata::<T>::get(&content_id).unwrap();

			site.promoted.push(content_id.clone());
			Sites::<T>::insert(site_id.clone(),site.clone());		

			// let mut promotionated_content = PromotionatedContent::<T>::get(&site_id).unwrap();
			// promotionated_content.push(content.clone());
			// PromotionatedContent::<T>::insert(site_id.clone(),promotionated_content);

			Self::deposit_event(Event::ContentPromoted(site_id.clone(), content_id.clone(), who.clone()));
	
			Ok(())
		}	

	}  

	impl<T: Config> Pallet<T> {

		fn increase_balance(acc: &T::AccountId, amount: T::Balance) -> bool {
			BalanceToAccount::<T>::mutate(&acc, |bal| {
				let created = bal == &Zero::zero();
				// fine because we check the issuance for overflow before minting and transfers
				// don't change the issuance
				*bal = bal.saturating_add(amount);
				created
			})
		}	
		
		fn decrease_balance(acc: &T::AccountId, amount: T::Balance) -> bool {
			BalanceToAccount::<T>::mutate(&acc, |bal| {
				let created = bal == &Zero::zero();
				// fine because we check the issuance for overflow before minting and transfers
				// don't change the issuance
				*bal = bal.saturating_sub(amount);
				created
			})
		}			

	}	

	
}