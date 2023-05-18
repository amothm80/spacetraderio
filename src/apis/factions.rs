use crate::apis::config;
use crate::apis::errors;
use crate::models::faction;
use crate::models::message;
use reqwest::Error;
use reqwest::Response;
//use serde_derive::{Serialize,Deserialize};
//use serde_json::Map;
//use serde_json::Value;
//use serde::Deserialize;

pub async fn get_factions(config: &config::Config) -> Result<Vec<faction::Faction>, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/factions",
    );
    //reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build().unwrap();
    let resp = client.execute(req).await.unwrap();
    let status = resp.status();
    let json = resp.json::<message::Message>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        if let message::Data::factions(a) = json.data {
            Ok(a)
        }else{
            Err(errors::STError::stgeneralerror)
        } 
    }else{
        Err(errors::STError::stapierror(json.error))
    }
    // resp.json::<message::Message>().await.unwrap();
    // match resp.json::<message::Message>().await? {
    //     message::Message::data(d) => match d {
    //         message::Data::faction(a) => Ok(a), //println!("{:#?}",a.accountId),
    //         //(_) => println!("{:#?}","error"),
    //         (_) => Err(errors::STError::stgeneralerror),
    //     },
    //     //message::Message::error(e) => println!("{:#?}",e),
    //     message::Message::error(e) => Err(errors::STError::stapierror(e)),
    // }








//     let j = r#"
// {"data":[{"symbol":"COSMIC","name":"CosmicEngineers","description":"TheCosmicEngineersareagroupofhighlyadvancedscientistsandengineerswhoseektoterraformandcolonizenewworlds,pushingtheboundariesoftechnologyandexploration.","headquarters":"X1-ZA40-15970B","traits":[{"symbol":"INNOVATIVE","name":"Innovative","description":"Willingtotrynewanduntestedideas.Sometimesabletocomeupwithcreativeandoriginalsolutionstoproblems,andmaybeabletothinkoutsidethebox.Sometimesattheforefrontoftechnologicalorsocialchange,andmaybewillingtotakerisksinordertoadvancetheboundariesofhumanknowledgeandunderstanding."},{"symbol":"BOLD","name":"Bold","description":"Unafraidtotakerisksandchallengethestatusquo.Sometimeswillingtodothingsthatotherswouldnotdare,andmaybeabletoovercomeobstaclesandchallengesthatwouldbeinsurmountableforothers.Sometimesabletoinspireandmotivateotherstotakeboldactionaswell."},{"symbol":"VISIONARY","name":"Visionary","description":"Possessingaclearandcompellingvisionforthefuture.Sometimesabletoseebeyondthepresentandanticipatetheneedsandchallengesoftomorrow.Sometimesabletoinspireandguideotherstowardsabetterandbrighterfuture,andmaybewillingtotakeboldanddecisiveactiontomaketheirvisionareality."},{"symbol":"CURIOUS","name":"Curious","description":"Possessingastrongdesiretolearnandexplore.Sometimesinterestedinawiderangeoftopicsandmaybewillingtotakerisksinordertosatisfytheircuriosity.Sometimesabletothinkoutsidetheboxandcomeupwithcreativesolutionstochallenges."}]},{"symbol":"VOID","name":"Voidfarers","description":"TheVoidfarersareagroupofnomadictradersandadventurerswhotravelthegalaxyinsearchofrichesandadventure,willingtotakerisksandexploretheunknown.","headquarters":"X1-DC54-63290D","traits":[{"symbol":"DARING","name":"Daring","description":"Willingtotakerisksandchallenges.Sometimesunafraidtoexplorenewandunknownterritories,andmaybewillingtotakeboldanddecisiveactioninordertoachievetheirgoals.Sometimesabletoovercomechallengesthatwouldbeinsurmountableforothers."},{"symbol":"EXPLORATORY","name":"Exploratory","description":"Dedicatedtoexplorationanddiscovery.Sometimesinterestedinmappingnewterritoriesanduncoveringthesecretsoftheuniverse.Sometimesabletoovercomeobstaclesandchallengesinordertoadvancetheboundariesofhumanknowledgeandunderstanding."},{"symbol":"RESOURCEFUL","name":"Resourceful","description":"Knownfortheiringenuityandabilitytomakethemostoutoflimitedresources.Abletoimproviseandadapttochangingcircumstances,usingwhateverisavailabletotheminordertoovercomechallengesandachievetheirgoals."},{"symbol":"FLEXIBLE","name":"Flexible","description":"Abletoadapttochangingcircumstancesandenvironments.Sometimesabletoquicklyswitchbetweendifferentstrategiesandtacticsinordertorespondtonewchallengesoropportunities.Sometimesabletoimproviseandthinkontheirfeet,makingthemdifficulttopredictoroutmaneuver."}]},{"symbol":"GALACTIC","name":"GalacticAlliance","description":"TheGalacticAllianceisacoalitionofplanetsandfactionsthathavebandedtogetherformutualprotectionandsupport,workingtogethertodefendagainstexternalthreatsandpromotecooperation.","headquarters":"X1-YP35-08710A","traits":[{"symbol":"COOPERATIVE","name":"Cooperative","description":"Willingtoworktogetherwithothersinordertoachievecommongoals.Sometimesabletocoordinateandcooperateeffectively,usingtheircollectivestrengthsandresourcestoovercomechallengesandachievesuccess.Oftenprioritizecollaborationandteamworkoverindividualachievement."},{"symbol":"UNITED","name":"United","description":"Stronglyunitedandcohesive,oftenwithastrongsenseofsharedidentityandpurpose.Sometimesabletoworktogethereffectivelyandefficiently,andmaybedifficulttodivideorconquer.Sometimesabletoovercomechallengesthatwouldbeinsurmountableforalessunitedgroup."},{"symbol":"PEACEFUL","name":"Peaceful","description":"Dedicatedtomaintainingpeaceandharmony.Sometimesreluctanttoengageinconflict,andmayprefertoresolvedisputesthroughnegotiationanddiplomacy.Sometimesabletocreateasenseofcommunityandbelonging,andmaybeabletobuildstrongandlastingrelationshipswithothers."},{"symbol":"STRATEGIC","name":"Strategic","description":"Skilledintheartofstrategyandplanning.Sometimesabletothinkaheadandanticipatetheactionsofothers,andmaybeabletodevelopeffectiveplanstoachievetheirgoals.Sometimesabletomakecalculatedrisksandsacrificesinordertogainastrategicadvantage."}]},{"symbol":"QUANTUM","name":"QuantumFederation","description":"TheQuantumFederationisagroupofplanetsandfactionsthathavejoinedtogethertoshareknowledgeandtechnology,usingtheircollectiveexpertisetoadvancethescienceandtechnologyofthegalaxy.","headquarters":"X1-ZS60-98730D","traits":[{"symbol":"INTELLIGENT","name":"Intelligent","description":"Possessingahighlevelofintelligenceandanalyticalability.Sometimesskilledinawiderangeoffields,includingscience,technology,andengineering.Oftenhaveastrongcuriosityandadesiretounderstandthemysteriesoftheuniverse."},{"symbol":"RESEARCH_FOCUSED","name":"Research-Focused","description":"Dedicatedtoadvancingknowledgeandunderstandingthroughresearchandexperimentation.Oftenhaveastrongfocusonscientificandtechnologicaldevelopment,andmaybewillingtotakerisksandexplorenewideasinordertomakeprogress."},{"symbol":"COLLABORATIVE","name":"Collaborative","description":"Knownfortheirabilitytoworkwellwithothers.Sometimeswillingtoshareresources,knowledge,andexpertiseinordertoachievecommongoals.Oftenhaveastrongsenseofcommunityandcooperation,andmayprioritizetheneedsofthegroupoverthoseoftheindividual."},{"symbol":"PROGRESSIVE","name":"Progressive","description":"Opentonewideasandchange.Sometimeswillingtoembracenewtechnologiesandwaysofthinking,andmayprioritizetheadvancementofknowledgeandunderstandingovertraditionandestablishedwaysofdoingthings."}]},{"symbol":"DOMINION","name":"StellarDominion","description":"TheStellarDominionisapowerful,expansionistfactionthatseekstoconquerandcontrolasmanysystemsaspossible,usingtheiradvancedtechnologyandmilitarymighttodominatetheirrivals.","headquarters":"X1-GZ13-43390C","traits":[{"symbol":"MILITARISTIC","name":"Militaristic","description":"Focusedonbuildingandmaintainingastrongmilitaryforce.Oftenprioritizemilitarypowerandreadinessoverotherconcerns,andmaybequicktoengageinconflictoraggressioninordertoachievetheirgoals."},{"symbol":"AGGRESSIVE","name":"Aggressive","description":"Quicktoengageinconflictoraggression,oftenwithoutprovocation.Sometimesunpredictableanddifficulttonegotiatewith,andmayprioritizetheirowninterestsovertheneedsofothers."},{"symbol":"IMPERIALISTIC","name":"Imperialistic","description":"Dedicatedtoexpandingtheirterritoryandinfluence.Oftenseektoconquerorsubjugateotherfactions,andmayhaveahierarchicalandauthoritarianstructure.Oftenprioritizetheinterestsoftheirownfactionovertheneedsofothers."},{"symbol":"INDUSTRIOUS","name":"Industrious","description":"Knownfortheirhardworkanddedication.Highlyproductiveandefficient,withafocusonmaximizingtheiroutput.Sometimesabletoproducelargequantitiesofgoodsorresources,butmayalsobevulnerabletoexploitationoroverwork."}]},{"symbol":"ASTRO","name":"Astro-SalvageAlliance","description":"TheAstro-SalvageAllianceisagroupofscavengersandsalvagerswhosearchthegalaxyforancientartifactsandvaluabletechnology,oftencombingthrougholdshipbattlegroundsandderelictspacestations.","headquarters":"X1-AC10-43560F","traits":[{"symbol":"SCAVENGERS","name":"Scavengers","description":"Skilledatfindingandsalvagingvaluableresourcesandmaterialsfromabandonedorderelictships,spacestations,andotherstructures.Resourcefulandabletomakethemostoutofwhatothershaveleftbehind."},{"symbol":"TREASURE_HUNTERS","name":"TreasureHunters","description":"Alwaysonthelookoutforvaluableartifacts,ancientrelics,andotherrareandvaluableitems.Curiousandwillingtotakerisksinordertouncoverhiddentreasuresandsecretsoftheuniverse."},{"symbol":"RESOURCEFUL","name":"Resourceful","description":"Knownfortheiringenuityandabilitytomakethemostoutoflimitedresources.Abletoimproviseandadapttochangingcircumstances,usingwhateverisavailabletotheminordertoovercomechallengesandachievetheirgoals."},{"symbol":"DEXTEROUS","name":"Dexterous","description":"Skilledintheuseoftheirhandsandabletoperformcomplextaskswithprecisionandaccuracy.Knownfortheirmanualdexterityandabilitytomanipulateobjectswithease,makingthemvaluableinawiderangeoftasksandactivities."}]},{"symbol":"CORSAIRS","name":"SeventhSpaceCorsairs","description":"TheSeventhSpaceCorsairsareafearedgroupofpiratesandraiderswhooperatethroughoutthegalaxy,preyingonmerchantshipsandplunderingvaluablecargo.","headquarters":"X1-T47-44290A","traits":[{"symbol":"UNPREDICTABLE","name":"Unpredictable","description":"Difficulttopredictoranticipate,withatendencytoactinunexpectedorchaoticways."},{"symbol":"BRUTAL","name":"Brutal","description":"Fierceandruthless,withawillingnesstouseviolenceorintimidationtoachievetheirgoals.Oftenfearedorrespectedbyothers,butmayalsobeviewedasathreatorenemybythosewhoopposetheirmethods."},{"symbol":"FLEETING","name":"Fleeting","description":"Notpermanentlysettledinoneplace,withatendencytomovefrequentlyorunpredictably.Sometimesdifficulttofindortrack,butmayalsobeabletotakeadvantageofopportunitiesorevadethreatsbymovingquicklyorunexpectedly."},{"symbol":"ADAPTABLE","name":"Adaptable","description":"Quicktoadapttochangingcircumstances,withtheabilitytoadjusttheirplansorstrategiesinresponsetonewinformationorchallenges.Sometimesabletothriveinawiderangeofenvironmentsorsituations,butmayalsobevulnerabletosuddenorunexpectedchanges."}]}],"meta":{"total":7,"page":1,"limit":10}}
//    "#;

   //"meta":{
    //"total":7,
    //"page":1,
    //"limit":10
// }

    // let deserializer = &mut serde_json::Deserializer::from_str(j);
    // let result: std::result::Result<message::MessagePaged, _> = serde_path_to_error::deserialize(deserializer);
    // match result {
    //     Ok(_) => println!("Expected an error"),
    //     Err(err) => {
    //         panic!("{},{}", err, err.path());
    //     }
    // }
    // let resp:message::Message = serde_json::from_str(j).unwrap();
    // println!("{:#?}", resp);
}

pub async fn get_faction(config: &config::Config, factionsymbol: String) -> Result<faction::Faction, errors::STError> {
    let client = &config.client;
    let mut reqbuilder = client.request(
        reqwest::Method::GET,
        config.base_path.to_owned() + "/factions/".to_owned().as_str() + factionsymbol.as_str(),
    );
    //reqbuilder = reqbuilder.bearer_auth(config.bearer_token.to_owned());
    let req = reqbuilder.build().unwrap();
    let resp = client.execute(req).await.unwrap();
    let status = resp.status();
    let json = resp.json::<message::Message>().await?;
    if !status.is_client_error() && !status.is_server_error() {
        if let message::Data::faction(a) = json.data {
            Ok(a)
        }else{
            Err(errors::STError::stgeneralerror)
        } 
    }else{
        Err(errors::STError::stapierror(json.error))
    }
}