use crate::creatures::{Creature, TypeCreature};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use std::borrow::BorrowMut;
use std::time::Duration;
use crate::SkellyAnimationId;

pub struct AnimationHandler;
impl Plugin for AnimationHandler {
    fn build(&self, app: &mut App) {
        app.insert_resource::<VecSceneHandle>(Default::default())
            .register_inspectable::<AnimationEntityLink>()
            .add_event::<ChangeAnimation>()
            .add_event::<AddAnimation>()
            .add_event::<RemoveAnimation>()
            .add_system(link_animations)
            .add_system(start_animation.after(link_animations))
            .add_system_to_stage(CoreStage::PostUpdate, add_animation)
            .add_system_to_stage(CoreStage::PostUpdate, remove_animation)
            .add_system_to_stage(CoreStage::PostUpdate, update_animation.after(add_animation))
            .add_system_to_stage(CoreStage::PostUpdate, checker_animation_duration.after(update_animation))
            //.add_system(inspect_animation_clip)
        ;
    }
}

/// Event utilisé par change_animation() pour changer d'animation
/// # Examples
/// ```
/// event_writer.send(
///    ChangeAnimation{
///        target: entity,
///        index: number as usize,
///        repeat: true,
///    }
/// );
/// ```
#[derive(Debug)]
pub struct ChangeAnimation {
    pub(crate) target: u32,
    pub(crate) index: usize,
    pub(crate) repeat: bool,
}

/// Event utilisé pour ajouter une animation
pub struct AddAnimation {
    pub scene_handler: SceneHandle,
    pub target: Option<u32>,
    pub start_animation: bool,
}

/// Event utilisé pour retirer une animation
pub struct RemoveAnimation {
    pub entity_id: u32,
}

#[derive(Component)]
pub struct TagPlayerScene;

/// Ressource qui contient un vecteur de SceneHandle
/// qui définit tous les animations des créatures
/// Ajouté au world:  app.insert_resource::<VecSceneHandle>(Default::default())
#[derive(Default)]
pub struct VecSceneHandle(pub Vec<SceneHandle>);

/// HashMap contenant un tuple: (duration_animation, handle_animation)
/// La Hashmap est créée dans la fonction spawn de chaque créature
/// Updated par add_animation et remove_animation
/// Utilisée par update_animation
#[derive(Clone, Debug)]
pub struct HashMapAnimationClip(HashMap<usize, (f32, Handle<AnimationClip>)>);

impl HashMapAnimationClip {
    pub fn get_pair(&self, ind: usize) -> Option<&(f32, Handle<AnimationClip>)> {
        self.0.get(&ind)
    }

    pub fn new() -> Self {
        HashMapAnimationClip(HashMap::new())
    }

    pub fn insert(
        &mut self,
        k: usize,
        duration: f32,
        handle: Handle<AnimationClip>,
    ) -> Option<(f32, Handle<AnimationClip>)> {
        self.0.insert(k, (duration, handle))
    }
}

/// utilisé par change_animation() pour mettre à jour la prochaine animation
#[derive(Component, Debug)]
pub struct AnimationStopWatch {
    /// if of the entity containing the scene
    pub creature_entity_id: u32,
    pub index_animation: usize,
    pub time: Timer,
    pub manual_termination: bool,
}

impl AnimationStopWatch {
    fn reset_timer(&mut self) {
        self.time.reset();
    }

    fn manual_is_over(&mut self) -> bool {
        if self.manual_termination {
            self.manual_termination = false;
            true
        } else {
            self.time.finished()
        }
    }


    fn tick(&mut self, delta: Duration) {
        self.time.tick(delta);
    }
}

#[derive(Clone, Debug)]
pub struct SceneHandle {
    /// handle of the scene
    pub handle: Handle<Scene>,

    /// vector of AnimationClip
    //pub vec_animations: Vec<Handle<AnimationClip>>,
    pub vec_animations: HashMapAnimationClip,

    /// if of the entity containing the scene
    pub creature_entity_id: Option<u32>,

    /// type of the creature. Same type can use the same AnimationClip & Scenes
    pub type_creature: TypeCreature,

    /// To handle mutilple scene for one entity
    pub activated: bool,
}

/// Composant qui mis à jour par link_animations()
/// L'entité est l'id de l'AnimationPlayer.
/// en utilisant ces deux queries :
/// ```
///    mut query_player: Query<&mut AnimationPlayer>,
///    mut query_entity: Query<(Entity, &AnimationEntityLink), With<Creature>>,
/// ```
/// On peut retrouver, pour une entité, son AnimationEntityLink, donc l'id de son AnimationPlayer
/// et:
/// ```
///    Ok(player) = query_player.get_mut(animation_link.0)
/// ```
#[derive(Component, Inspectable)]
pub struct AnimationEntityLink(pub Entity);

impl AnimationEntityLink {
    fn get(&self) -> Entity {
        self.0
    }
}

fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    while let Ok(parent) = parent_query.get(curr_entity) {
        curr_entity = parent.get();
    }
    curr_entity
}

/// Fonction qui lie une entité avec son AnimationPlayer par le composant AnimationEntityLink.
/// Voir: https://github.com/bevyengine/bevy/discussions/5564#discussion-4275825
fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the hierarchy

    for entity in player_query.iter() {
        let skelly_entity = get_top_parent(entity, &parent_query);

        debug!("Calling: link_animations. {:#?}", entity);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(skelly_entity).is_ok() {
            warn!("Problem with multiple animations players for the same top parent");
            warn!(
                "{:?} {:?} AnimationLink.{:?}",
                entity,
                skelly_entity,
                animations_entity_link_query.get(skelly_entity).unwrap().0
            );
        } else {
            warn!("Skelly {:?} / AnimationPlayer:{:?})", skelly_entity, entity);
            commands
                .entity(skelly_entity)
                .insert(AnimationEntityLink(entity));
        }
    }
}

/// Une fois que link_animations() a ajouté un AnimationEntityLink :
/// Lancer la première animation !
fn start_animation(
    query_entity: Query<Entity, (With<Creature>, Added<AnimationEntityLink>)>,
    mut writer: EventWriter<ChangeAnimation>,
) {
    for entity in query_entity.iter() {
        writer.send(ChangeAnimation {
            target: entity.id(),
            index: 0,
            repeat: false,
        })
    }
}

/// Fonction qui lit un Event ChangeAnimation et :
///   1. D'après l'id de l'entité à animer (event.target.id())
///   2. Retrouver l'animationPlayer associé en parcourant les tuples (Entity, &AnimationEntityLink)
///      ```
///         creature.id() == event_creature_à_animer.target
///      ```
///   3. Une fois le player retrouvé, on cherche les animations dans VecSceneHandle
///      ```
///         scene_handler_random_creature.id() == event_creature_à_animer.id()
///      ```
fn update_animation(
    mut events: EventReader<ChangeAnimation>,
    scene_handlers: Res<VecSceneHandle>,
    mut query_player: Query<&mut AnimationPlayer>,
    mut query_entity: Query<(Entity, &AnimationEntityLink, &mut Creature)>,
    mut query_stopwatch: Query<&mut AnimationStopWatch>,
) {
    for event in events.iter() {
        // retrouver l'entity
        debug!("Event found! {:#?}", event);
        for (entity, animation_link, mut creature) in query_entity.iter_mut() {
            if entity.id() == event.target {
                // on a retrouvé le player associé à l'entité
                debug!("  > entity trouvé!");
                for scene_handler in &scene_handlers.0 {
                    // the second condition should be enough.
                    if scene_handler.activated
                        && scene_handler.type_creature == creature.type_creature
                    /* scene_handler.creature_entity_id == Some(entity.id()) */
                    {
                        // on retrouve ses animations SceneHandler
                        debug!(
                            "  > scene_handler trouvé pour {:#?}",
                            scene_handler.type_creature
                        );

                        if let Ok(mut player) = query_player.get_mut(animation_link.get()) {
                            let mut event_index =event.index;

                            let mut duration = &0.0;
                            let mut animation = &bevy::asset::Handle::default();

                            if creature.type_creature != TypeCreature::SkellyFullBody {
                                /*
                                    Big hack, but not time to make it reasonable

                                    IdleIndex for NonFullBody is 0
                                    For FullBody, it is SkellyAnimationId::Idle == 1

                                    But need to set creature.current_animation_index to SkellyAnimationId::Idle
                                    because I coded it like that..
                                 */
                                warn!("Found and block to 0 !");
                                event_index = SkellyAnimationId::Idle as usize;

                                // god now I'm struggling with rust, help me
                                let a =
                                    scene_handler.vec_animations.get_pair(0).unwrap();
                                duration = &a.0;
                                animation = &a.1;
                            } else {
                                let a  =
                                    scene_handler.vec_animations.get_pair(event_index).unwrap();
                                duration = &a.0;
                                animation = &a.1;
                            }
                            creature.current_animation_index.0 = event_index;

                            if event.repeat {
                                player.play(animation.clone_weak()).repeat();
                                debug!("Playing repeat!");
                            } else {
                                player.play(animation.clone_weak());
                                debug!("Playing!");
                            }

                            for mut stopwatch in query_stopwatch.iter_mut() {
                                if stopwatch.creature_entity_id == entity.id() {
                                    stopwatch.index_animation = event_index;
                                    stopwatch
                                        .time
                                        .set_duration(Duration::from_secs_f32(*duration));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Utiliser pour trouver la durée d'une AnimationClip
/// Ne fonctionne que pour le premier élément ajouté dans VecSceneHandle, après 'done' > 2.
/*
fn inspect_animation_clip(
    assets_handle: Res<Assets<AnimationClip>>,
    scene_handlers: Res<VecSceneHandle>,
    mut done: Local<usize>,
) {
    if *done > 2 {
        return
    }

    for scene_handler in &scene_handlers.0 {
        for ind in 0..scene_handler.vec_animations.0.len() {
            let (_, handle) = scene_handler.vec_animations.get_pair(ind).unwrap();
            if let Some(anim) = assets_handle.get(handle)
            {
                let st = format!("Start : {:?}", anim);
                let duration_str_i = (st.as_str()).find("duration").unwrap();
                let len = st.len();

                info!("Anim index {}, {}", ind, st.get(duration_str_i..len-2).unwrap());
                *done += 1;
            }
        }
    }

}
*/

/// Récupère les stopwatch
/// Met à jour les ticks des stopwatch
/// Si une stopwacth est terminée :
///     Une animation est terminée
///     Récupérer la créature de l'animation et appeler sa fonction update_animation() pour choisir la prochaine animation
fn checker_animation_duration(
    query_entity: Query<(Entity, &Creature)>,
    mut query_stopwatch: Query<&mut AnimationStopWatch>,
    mut event_writer: EventWriter<ChangeAnimation>,
    time: Res<Time>,
) {
    for mut stopwatch in query_stopwatch.iter_mut() {
        stopwatch.tick(time.delta());

        if stopwatch.manual_is_over() {
            // play new animation for the current entity
            debug!("Timer finished for entity {}", stopwatch.creature_entity_id);
            stopwatch.reset_timer(); // en attendant que update_animation vienne faire le travail

            let index_animation = stopwatch.index_animation;

            for (entity, creature) in query_entity.iter() {
                if entity.id() == stopwatch.creature_entity_id {
                    creature.update_animation(
                        stopwatch.creature_entity_id,
                        index_animation,
                        event_writer.borrow_mut(),
                    );
                }
            }
        }
    }
}

fn add_animation(
    mut events: EventReader<AddAnimation>,
    mut vec_scene_handlers: ResMut<VecSceneHandle>,
    mut commands: Commands,
) {
    for event in events.iter() {
        debug!("AddAnimation: {:#?}", event.scene_handler);

        vec_scene_handlers.0.push(event.scene_handler.clone());

        // On ajoute une Stopwatch si on démarre l'animation
        if event.start_animation {
            let target = event.target.expect(
                "Add_Animation a été appelé avec start_animation==true sans entity en target!",
            );
            spawn_animation_stop_watch(target, 0, commands.borrow_mut());
        }
    }
}


/// Ajoute une stopwatch au World.
/// Les stopwatch gardent le temps actuel de l'animation en cours pour une créature
/// Mis à jour par checker_animation_duration()
/// creature_entity_id ne doit pas être une Option! Obligatoire.
pub fn spawn_animation_stop_watch(
    creature_entity_id: u32,
    index_animation: usize,
    commands: &mut Commands,
) {
    commands
        .spawn()
        .insert(AnimationStopWatch {
            creature_entity_id,
            index_animation,
            time: Timer::new(Duration::from_secs(1000.0 as u64), false),
            manual_termination: false,
        })
        .insert(Name::new(format!("Stopwatch {}", creature_entity_id)));
}

fn remove_animation(
    mut events: EventReader<RemoveAnimation>,
    mut vec_scene_handlers: ResMut<VecSceneHandle>,
) {
    let mut found;
    for event in events.iter() {
        found = false;
        for i in 0..vec_scene_handlers.0.len() {
            if !found && vec_scene_handlers.0[i].creature_entity_id == Some(event.entity_id) {
                debug!("Remove_animation: entity found, removing.");
                found = true;
                vec_scene_handlers.0.swap_remove(i);
            }
        }
        if !found {
            warn!("Remove_animation: called with unknown entity")
        }
    }
}
