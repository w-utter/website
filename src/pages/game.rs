use crate::components::counter_btn::Button;
use leptos::prelude::*;
use leptos_floating::*;

//TODO: make it not look bad & add media

#[component]
pub fn Game() -> impl IntoView {

    let (game_state, set_game_state) = signal(GameState::Begining);

    provide_context(game_state);

    view!{
        <div style:width="100vw" style:height="100vh" style:display="flex" style:justify-content="center" style:align-content="center" >
            <div style:width="80%" style:height="100%" style:text-align="left">
                {GameState::as_view(set_game_state)}
            </div>
        </div>
    }
}

#[derive(Clone)]
enum GameState {
    Begining,
    Section1(usize),
    Section2(String),
    Section3(ReplayingState, bool),
    Section4(Vec<u8>),
    Section5(String, bool),
    End,
}

#[component]
fn Section1(on_phrase_click: impl Fn(leptos::ev::MouseEvent) + 'static, title: &'static str) -> impl IntoView {
    view!{
        <Title content=move || view!{ <Header content=title/>} />

        <div> Puzzles in video games suck. Something that should be used to reward the player for <Clickable text_content="thoughtfulness" content=|| "b"/> is more often than not used as a lazy mechanic to explore other areas. The amount of times that Ive seen a <Clickable text_content="colored lock" content=|| "a"/> only to have to  <button style:all="unset" on:click=on_phrase_click>lope</button> around on an excursion to find a key of matching color, completely ignoring the story thats being told in order to get to the next section of the game. I can only stand so many -- what seems like -- fetch quests in games without it feeling drawn out and unnecessary to the development of whatever is going on.</div>
    }
}

fn update_section_1_count(signal: WriteSignal<GameState>) -> impl Fn(leptos::ev::MouseEvent) {
    move |_| {
        signal.maybe_update(|state| {
            let GameState::Section1(count) = state else {
                return false;
            };
            if *count + 1 == 22 {
                *state = GameState::Section2(String::new())
            } else {
                *count += 1
            }
            true
        })
    }
}

fn update_section_3_count(signal: WriteSignal<GameState>) -> impl Fn(leptos::ev::MouseEvent) {
    move |_| {
        signal.maybe_update(|state| {
            let GameState::Section3(replaying, found) = state else {
                return false;
            };

            let ReplayingState::Replaying(count) = replaying else {
                return false;
            };

            if *count + 1 == 22 {
                if *found {
                    *state = GameState::Section4(vec![]);
                } else {
                    *replaying = ReplayingState::Finished;
                }
            } else {
                *count += 1;
            }
            true
        })
    }
}

impl GameState {
    fn as_view(set: WriteSignal<Self>) -> impl IntoView {
        let state = use_context::<ReadSignal<GameState>>().unwrap();
        let to_section_1 = move |_| set(GameState::Section1(0));

        leptos_use::use_event_listener(leptos_use::use_document(), leptos::ev::keydown, move |ev| {
            set.maybe_update(|state| {

                if matches!(ev.key_code(), 0x0D) {
                    // Enter
                    match state {
                        GameState::Section2(a) => {
                            if matches!(a.trim(), "red herring") {
                                *state = GameState::Section3(ReplayingState::NotFound, false);
                            } else {
                                a.clear()
                            }
                            return true;
                        }
                        GameState::Section5(a, true) => {
                            if matches!(a.trim(), "bad design") {
                                *state = GameState::End
                            } else {
                                a.clear()
                            }
                            return true;
                        }
                        _ => return false,
                    }
                } else  {
                    let accum = match state {
                        GameState::Section2(a) => a,
                        GameState::Section5(a, true) => a,
                        _ => return false,
                    };

                    if matches!(ev.key_code(), 0x08 | 0x2E ) {
                        // Delete / backspace
                        accum.pop();
                    } else {
                        if ev.key().chars().count() > 1 {
                            return false;
                        }
                        accum.push_str(&ev.key());
                    }
                }
                return true;
            })
        });

        view! {
            {
                move || match *state.read() {
                    Self::Begining => view! {   
                        <Title content=|| view!{ <Header content="A complaint about puzzle games"/>} />

                        <div> I was going to make a video for this project but it seems that someone has turned my script into a puzzle to prove a point or something. The original is being held by ransomware and I do not have enough crypto to give them (fell asleep in the one lecture about game markets & economies). It also seems that whoever made it way too difficult and I cannot solve it. Below links to the website. If anyone is able to solve it, please let me know!</div>

                        <div 
                            style:width="100%" 
                            style:display="flex" 
                            style:justify-content="center"
                        >
                            <button on:click=to_section_1>begin</button>
                        </div>
                    }.into_any(),
                    Self::Section1(count) => {
                        view!{
                            <Section1 on_phrase_click=update_section_1_count(set) title="A complaint about puzzle games" />
                        }.into_any()
                    }
                    Self::Section2(ref phrase) => {
                        let phrase = phrase.clone();
                        let phrase2 = phrase.clone();
                        view! {
                            <Title content=|| view!{ <Header content="A complaint about puzzle games"/>} />

                            <div> At the extreme of this there are people who, whenever a puzzle comes up in a game they play, immediately pull up a <ToolTip text_content="video guide" tooltip_content=|| "not one here sorry :/"/> regardless of how difficult it may be -- which I dont think is the fault of the player. I think that its a lack of interest that drives people to want to look up a solution: if a puzzle is not rewarding enough, or <ToolTip text_content="too vague" tooltip_content=|| "start pressing keys" /> in how it is to be solved, then obviously people will become frustrated and choose the easy way out. Ive experienced something similar where sometimes I cannot find a particular object, whether it was just in a hard to see place or unknowingly blocked behind another puzzle/section which was not conveyed whatsoever to the person playing the game! What Im trying to get at is that disengaging puzzles is an issue with the way that they are currently being implemented in games. Puzzles should be rewarding to solve and have at least some general direction to lead the player in; with some sort of <ToolTip text_content="reinforcement" tooltip_content=|| "not the right place to look!" /> to tell people that they are either going in the right direction. On the other hand, if most puzzles in a game are too easy for the player, it feels infantilizing; like a kid who cannot be trusted to <ToolTip text_content="discover things themselves"  tooltip_content=|| "colors and shapes!"/> . </div>

                            <Show when=move || !phrase.is_empty()>
                                <div 
                                    style:position="fixed"
                                    style:left="0"
                                    style:top="0"
                                    style:width="100vw"
                                    style:height="100vh"
                                    style:background="rgba(0,0,0,0.5)"
                                    style:align-content="center"
                                    style:align-items="center"
                                    style:justify-content="center"
                                    style:display="flex"
                                    style:backdrop-filter="blur(0.25em)"
                                >
                                    {format!("{phrase2}")}
                                </div>
                            </Show>
                        }.into_any()
                    }
                    Self::Section3(replaying, found) => {
                        let count = if matches!(replaying, ReplayingState::Finished) || found {
                            format!("1/2")
                        } else {
                            format!("0/2")
                        };

                        if let ReplayingState::Replaying(count) = replaying {
                            view!{
                                <Section1 on_phrase_click=update_section_3_count(set) title="Again ?" />
                            }.into_any()
                        } else {
                            view!{
                                <Title content=move || view!{ <div on:click=move |_| if matches!(replaying, ReplayingState::Finished) {set(GameState::Section4(vec![]))} else if !found { set(GameState::Section3(replaying, true))}><Header content="A complaint about puzzle games"/></div>} />
                                <div on:click=move |_| {if !matches!(replaying, ReplayingState::Finished) { set(GameState::Section3(ReplayingState::Replaying(0), found)) }}>Another issue is that a well designed puzzle does not have good replayability. A puzzle that is engaging, in my opinion, should incorporate a portion of the story into its solution, while using the environment to hint as to what to do next. A puzzle in a game that does this well is the Infirmary puzzle in Penumbra: Black Plague. Players approach a locked door with medical equipment strewn throughout the room, where they need to find and inject themselves with ^Viscae Somnus^, which slows the subjects heart to such a slow rate that it appears dead. After the player blacks out due to the chemical injection, the narrator -- the virus that is carried through the player for the entire game which can affect your perception of the environment -- in the cutscene says ^I really thought that mightve been curtains^. This puzzle foreshadows the ending of the game, where the player takes another series of chemicals, except that the narrator is separated from the protagonist into a tuurngait which ends up killing them. This was intentionally designed by the developers to fit into the narrative as they have said on their website. However, during a second playthrough, the items are in the exact same spot, with the exact same sequence of events needing to take place.  Unless a game is able to change its story & environment with the flick of the wrist, it doesnt look like the issue of replayability is going to be solved any time soon. But I would rather have a really good first playthrough rather than a couple mediocre playthroughs of the same game. This can also be solved by just adding predetermined variations of the environment to some extent, but the approach to solving the puzzle will always remain relatively the same, which is the bigger issue at hand. If the player knows what motions to go through to be able to solve the puzzle it defeats the purpose of engaging with the story & environment to discover clues which can make the game feel repetitive and withdrawn, especially if this is the first playthrough. </div>
                                <br/>
                                <div> Introducing nonlinearity into puzzles also isnt going to fix this issue however. It feels more like the solution in most games is so <ToolTip text_content="in front of your face" tooltip_content=|| "you can find it somewhere earlier" /> that it doesnt matter if youre able to complete tasks in a different order. A lot of games, particularly horror-related, feel like theres more often than not one very obvious solution to any puzzle seen in games, even if the way of completing them can be done in whatever order you please. One of the games in particular that came up was Silent Hill 2 -- I was thinking of the original but the remaster also works. One of the earlier parts of the game requires exploring around the town of Silent Hill to get familiar with the environment. There is a puzzle which requires multiple coins which are scattered throughout the town. One of the coins is in a certain section of the woodside apartments which can only be gotten after completing & exploring other puzzles around and dropping a box of juice onto the trash shoot. Another one of the coins can be found in a stroller in a courtyard from collecting a different key from a different section of the woodside apartments, for instance. All of this can be done before even knowing what the coins are used for, as the puzzle is not brought up until a later section of the game. However, once the places for each of the coins is known, and their use is revealed, it becomes the same scenario where the player needs to look for an item to progress, it just might be scattered throughout multiple parts of the map. There isnt anything particularly wrong with this kind of puzzle, Silent Hill has been known for some incredible puzzles (along with some that are not so good), but it feels like if the hint that is given for the puzzle is something material rather than how to correctly interact with the puzzle it feels more of a chore than something that is rewarding. I would much rather struggle with trying to figure out how the engravings on the wall correspond to what the clock should face (one of the first puzzles in Silent Hill 2) than run around looking for items to put into an item-shaped hole. </div>

                                <div
                                    style:position="fixed"
                                    style:left="0"
                                    style:bottom="0"
                                    style:padding="1.5em"
                                >
                                    {count}
                                </div>
                            }.into_any()
                        }
                    }
                    Self::Section4(_) => {
                        view!{
                            <Title content=|| view!{ <Header content="A complaint about puzzle games"/>} />
                            <div> A particular genre of game that feels more like what I want from games is alternate reality games (ARG), where the media in which is presented is thoroughly dissected, often by entire communities, to uncover whatever secrets may be hiding beneath its surface. With no guides, and often very little direction, a lot of the time is spent theorycrafting outside of the game on how to decipher the meaning of what is being told, which increases retention and creates a sense of depth that isnt seen often in video games in the traditional sense. Courage the Cowardly Dog ARG does this really well. Essentially, someone finds an old PS1 Courage game at a yard sale and uploads its contents to itch.io for anyone to play, however there is a <Clickable text_content="hidden story" content=|| "old wind symbol" /> that is uncovered while playing -- all which refer back to <Clickable text_content="\"old wind\"" content=move|| view!{<Clock signal=set/>} /> which is never directly acknowledged by the game. Using a various number of media editing software, in order to increase brightness on images to show hidden text, using a spectrogram to view hidden messages in audio files, analyzing audio for morse code, or even looking for hidden files in the game save data, gives clues and partial solutions to the puzzles throughout the game, and connects to the story that the creator is trying to establish. There doesnt actually exist a PS1 game made by the Courage The Cowardly dog franchise -- this isnt a ben drowned kinda situation -- it exists as the medium to use the show to reference and fall back on at times, while uncovering a different story. There are a couple full story/game analyses up on Youtube that go much more in depth about this than Im going to cover here. But regardless, ARGs more often than not require work to be done outside of the medium that theyre being presented, even for some that arent uploaded as games such as the Courage game, which encourage thinking more about the game and how you interact with it, and how the game community can come together to solve the mystery of the game. The community aspect particularly for ARGs is so that everyone invested in the game is able to have the same amount of information to try and solve it collectively which doesnt always exist for the other games mentioned (except when games first come out) but more often than not the puzzles in ARGs are <ToolTip text_content="much more difficult" tooltip_content=|| "align the two circles"/> than those in the other games covered. Regardless, the way which puzzles are used in ARGs  should be taken into consideration when designing other sorts of puzzles in games, as they tend to be difficult enough to engage the community while adequately rewarding those who solve them. </div>
                        }.into_any()
                    }
                    Self::Section5(ref phrase, back) => {
                        let phrase = phrase.clone();
                        let phrase2 = phrase.clone();
                        if back {
                            view!{
                                <Title content=|| view!{ <Header content="A complaint about puzzle games"/>} />

                                <div> Another issue is that a well designed puzzle does not have good replayability. A puzzle that is engaging, in my opinion, should incorporate a portion of the story into its solution, while using the environment to hint as to what to do next. A puzzle in a game that does this well is the Infirmary puzzle in Penumbra: Black Plague. Players approach a locked door with medical equipment strewn throughout the room, where they need to find and inject themselves with Viscae Somnus, which slows the subjects heart to such a slow rate that it appears dead. After the player blacks out due to the chemical injection, the narrator -- the virus that is carried through the player for the entire game which can affect your perception of the environment -- in the cutscene says ^^I really thought that mightve been curtains^^ This puzzle foreshadows the ending of the game, where the player takes another series of chemicals, except that the narrator is separated from the protagonist into a tuurngait which ends up killing them. This was intentionally designed by the developers to fit into the narrative as they have said on their website. However, during a second playthrough, the items are in the exact same spot, with the exact same sequence of events needing to take place.  Unless a game is able to change its story & environment with the flick of the wrist, it doesnt look like the issue of replayability is going to be solved any time soon. But I would rather have a really good first playthrough rather than a couple mediocre playthroughs of the same game. This can also be solved by just adding predetermined variations of the environment to some extent, but the approach to solving the puzzle will always remain relatively the same, which is the bigger issue at hand. If the player knows what motions to go through to be able to solve the puzzle it defeats the purpose of engaging with the story & environment to discover clues which can make the game feel repetitive and withdrawn, especially if this is the first playthrough. </div>

                                <br/>

                                <div> Introducing nonlinearity into puzzles also isnt going to fix this issue however. It feels more like the solution in most games is so in front of your face that it doesnt matter if youre able to complete tasks in a different order. A lot of games, particularly horror-related, feel like theres more often than not one very obvious solution to any puzzle seen in games, even if the way of completing them can be done in whatever order you please. One of the games in particular that came up was Silent Hill 2 -- I was thinking of the original but the remaster also works. One of the earlier parts of the game requires exploring around the town of Silent Hill to get familiar with the environment. There is a puzzle which requires multiple coins which are scattered throughout the town. One of the coins is in a certain section of the woodside apartments which can only be gotten after completing & exploring other puzzles around and dropping a box of juice onto the trash shoot. Another one of the coins can be found in a stroller in a courtyard from collecting a different key from a different section of the woodside apartments, for instance. All of this can be done before even knowing what the coins are used for, as the puzzle is not brought up until a later section of the game. However, once the places for each of the coins is known, and their use is revealed, it becomes the same scenario where the player needs to look for an item to progress, it just might be scattered throughout multiple parts of the map. There isnt anything particularly wrong with this kind of puzzle, Silent Hill has been known for some incredible puzzles (along with some that are not so good), but it feels like if the hint that is given for the puzzle is something material rather than how to correctly interact with the puzzle it feels more of a chore than something that is rewarding. I would much rather struggle with trying to figure out how the engravings on the wall correspond to what the clock should face (one of the first puzzles in Silent Hill 2) than run around looking for items to put into an item-shaped hole. </div>

                                <button 
                                    style:position="fixed"
                                    style:left="0"
                                    style:bottom="0"
                                    style:padding="1.5em"

                                    on:click=move |_| set.update(|state| {
                                    let GameState::Section5(_, back) = state else {
                                        unreachable!();
                                    };
                                    *back = false;
                                })>">"</button>
                                <Show when=move || !phrase.is_empty()>
                                    <div 
                                        style:position="fixed"
                                        style:left="0"
                                        style:top="0"
                                        style:width="100vw"
                                        style:height="100vh"
                                        style:background="rgba(0,0,0,0.5)"
                                        style:align-content="center"
                                        style:align-items="center"
                                        style:justify-content="center"
                                        style:display="flex"
                                        style:backdrop-filter="blur(0.25em)"
                                    >
                                        {format!("{phrase2}")}
                                    </div>
                                </Show>
                            }.into_any()
                        } else {
                            let audio_ref = NodeRef::<leptos::html::Audio>::new();
                            let play_music = move |_| {
                                let Some(audio) = audio_ref.get() else {
                                    return;
                                };
                                audio.play();
                            };

                            view!{
                                <audio node_ref=audio_ref loop controls>
                                    <source src=""/>
                                </audio>
                                <Title content=|| view!{ <Header content="A complaint about puzzle games"/>} />
                                <div>
                                    Thats not to say that simpler puzzles dont have their place in games. If every corner I turned in Resident Evil made me solve one of the millenium questions while some beast beyond my comprehension is slowly approaching me from behind I dont think I would want to play Resident Evil anymore. Im saying that theres a time and a place to have engaging puzzles in games, similar to how cutscenes are used to break up pacing, puzzles (usually) give a more laid back setting for the player to discern whats going on and think about the story that theyve been put into. Allowing time to stop and think during a puzzle, rather than running around aimlessly looking for the solution, aids in driving whatever atmosphere the designers intended; taking in the <button style:all="unset" on:click=play_music>music & ambient noise</button>  while being immersed in the environment with no other task but to figure out how continue onto the next portion of the game with the context theyre given.
                                </div>

                                <button 
                                    style:position="fixed"
                                    style:left="0"
                                    style:bottom="0"
                                    style:padding="1.5em"

                                    on:click=move |_| set.update(|state| {
                                        let GameState::Section5(_, back) = state else {
                                            unreachable!();
                                        };
                                        *back = true;
                                })>"<"</button>
                            }.into_any()
                        }
                    },
                    Self::End => view! {
                        <Title content=|| view!{ <Header content="Congratulations !! 🎉"/>} />

                        <div> Still, the philosophy of how puzzles are integrated into modern games could be much better. A lot of the time when playing certain games it feels like solutions are discovered by accident or brute force. I want to feel rewarded when I figure out how to complete a section of a game, and the way which puzzles are integrated in most games do not try to incorporate this into the design of their game. </div>

                        <div>Looks like you dont have to pay to access the original</div>

                        <br/>

                        <div>the full document can be found <a href="https://" target="_blank">Here</a> if you arent sick of reading it already</div>
                    }.into_any()
                }
            }
        }
    }
}

#[component]
fn Clock(signal: WriteSignal<GameState>) -> impl IntoView {
    let reference_ref = NodeRef::<leptos::html::Div>::new();

    let state = use_context::<ReadSignal<GameState>>().unwrap();

    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();

        let Some(node) = reference_ref.get() else {
            return;
        };

        let a = node.get_bounding_client_rect();
        let mouse_x = ev.client_x();
        let mouse_y = ev.client_y();
        let node_x = a.x();
        let node_y = a.y();
        let node_width = a.width();
        let node_radius = node_width / 2.;

        let node_center_x = (node_x + node_radius) as i32;
        let node_center_y = (node_y + node_radius) as i32;
        let radius = node_radius as i32;


        let y = (mouse_y - node_center_y) as f32;
        let x = (mouse_x - node_center_x) as f32;

        let angle = f32::atan2(y, x) * 180. / std::f32::consts::PI;

        let clock_num = if angle < 10. && angle > -10. {
            3
        } else if angle < 40. && angle > 20. {
            4
        } else if angle < 70. && angle > 50. {
            5
        } else if angle < 100. && angle > 80. {
            6
        } else if angle < 130. && angle > 110. {
            7
        } else if angle < 160. && angle > 140. {
            8
        } else if angle < -170. || angle > 170. {
            9
        } else if angle > -160. && angle < -140. {
            10
        } else if angle > -130. && angle < -110. {
            11
        } else if angle > -100. && angle < -80. {
            12
        } else if angle > -70. && angle < -50. {
            1
        } else if angle > -40. && angle < -20. {
            2
        } else {
            return
        };

        let dist = f32::sqrt(x*x + y*y);

        if dist as i32 > radius || dist < (radius as f32) * (3./4.) {
            return;
        }

        signal.update(|state| {
            let GameState::Section4(entries) = state else {
                unreachable!();
            };

            entries.push(clock_num);

            if entries.len() == 5 {
                if matches!(entries.as_slice(), &[5, 1, 8, 3, 12]) {
                    *state = GameState::Section5(String::new(), false)
                } else {
                    entries.clear();
                }
            }
        });
    };

    fn derive_num(state: ReadSignal<GameState>, idx: usize) -> Signal<Option<u8>> {
        Signal::derive(move || {
            let GameState::Section4(ns) = state.get() else {
                unreachable!()
            };
            ns.get(idx).copied()
        })
    }
    let num1 = derive_num(state, 0);
    let num2 = derive_num(state, 1);
    let num3 = derive_num(state, 2);
    let num4 = derive_num(state, 3);
    let num5 = derive_num(state, 4);

    view!{
        <div
            style:display="flex"
            style:height="100%"
            style:justify-content="center"
            style:flex-direction="column"
            on:click=|e| e.stop_propagation()
        >
            <div
                style:display="flex"
                style:flex-direction="row"
                style:justify-content="center"
                on:click=|e| e.stop_propagation()
            >
                <Digit num={num1}/>
                <Digit num={num2}/>
                <Digit num={num3}/>
                <Digit num={num4}/>
                <Digit num={num5}/>
            </div>
            <div 
                node_ref=reference_ref
                style:border-radius="100%"
                style:aspect-ratio="1"
                style:height = "75%"
                style:background="white"
                on:click=on_click
            >
            </div>
        </div>
    }
}

#[component]
fn Digit(num: Signal<Option<u8>>) -> impl IntoView {
    view! {
        <div
            style:min-width="1em"
            style:margin="0 1em"
            style:padding="0.25em 0.5em"
            style:background="rgba(0,0,0,0.5)"
        >
            {move || num.get().map(|n| format!("{n}")).unwrap_or_default()}
        </div>
    }
}



#[derive(Clone, Copy)]
enum ReplayingState {
    NotFound,
    Replaying(usize),
    Finished,
}

#[component] 
fn ToolTip<IV: IntoView + 'static>(text_content: &'static str, tooltip_content: impl Fn() -> IV + Send + Sync + 'static) -> impl IntoView {
    let reference_ref = NodeRef::new();
    let floating_ref = NodeRef::new();

    let UseFloatingReturn { x, y, .. } = use_floating(
        reference_ref,
        floating_ref,
        FloatingOptions {
            side: Side::Top,
            align: Align::Center,
            side_offset: 15.,
            ..Default::default()
        },
    );

    let is_hovered = leptos_use::use_element_hover(reference_ref);

    view! {
        <button node_ref=reference_ref style:all="unset" >{text_content}</button>

        <Show when=is_hovered>
            <div
                node_ref=floating_ref
                style:position="fixed"
                style:left=move || format!("{}px", x.get())
                style:top=move || format!("{}px", y.get())
                class="no-events"
                style:background="rgba(0,0,0,0.5)"
                style:padding="0 1em"
                style:border-radius="5px"
                style:backdrop-filter="blur(0.25em)"
            >
                {tooltip_content()}
            </div>
        </Show>
    }
}

#[component]
fn Title<IV: IntoView + 'static>(content: impl Fn() -> IV + Send + Sync + 'static) -> impl IntoView {
    view! {
        <div style:text-align="center" style:padding="1em">
            {content()}
        </div>
    }
}

#[component]
fn Header(content: &'static str) -> impl IntoView {
    view!{ 
        <h1>{content}</h1>
    }
}

#[component]
fn Clickable<IV: IntoView + 'static>(text_content: &'static str, content: impl Fn() -> IV + Send + Sync + 'static) -> impl IntoView {
    let (show, set_show) = signal(false);

    view!{
        <button style:all="unset" on:click=move |_| set_show(true) >{text_content}</button>
        <Show when=show>
            <div 
                style:position="fixed"
                style:left="0"
                style:top="0"
                style:width="100vw"
                style:height="100vh"
                style:background="rgba(0,0,0,0.5)"
                style:align-content="center"
                style:align-items="center"
                style:justify-content="center"
                style:display="flex"
                style:backdrop-filter="blur(0.25em)"
                on:click=move|_| set_show(false)
            >
                {content()}
            </div>
        </Show>
    }
}
