extern crate svgbob;

use svgbob::{CellBuffer, Node};
pub use svgbob::Settings;
use uuid::Uuid;

/// convert bob ascii diagrams to svg
pub fn bob_handler(s: &str, settings: &Settings) -> String {
	let cb = CellBuffer::from(s);
	let (svg, _, _): (Node<()>, f32, f32) = cb.get_node_with_size(settings);

	let mut source = String::new();
	svg.render_with_indent(&mut source, 0, true).expect("html render");

    let uuid = Uuid::new_v4().to_simple().to_string();
	format!("{}", source).replace("svgbob", &format!("svgbob_{}", uuid))
}

fn main() {
    // 粘贴测试用的svgbob数据
    let diagram_a = r#"
Svgbob is a diagramming model
which uses a set of typing characters
to approximate the intended shape.

       .---.
      /-o-/--
   .-/ / /->
  ( *  \/
   '-.  \
      \ /
       '
It uses a combination of characters
which are readily available on your keyboards.

What can it do?

 o->  Basic shapes
    .- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -.
    !                                                    .            :
    !   +------+   .------.    .------.      /\        .' `.          :
    !   |      |   |      |   (        )    /  \     .'     `.   ^    :
    !   +------+   '------'    '------'    '----'     `.   .'   /   # :
    !     _______            ________               #   `.'    / ^ /  :
    !    /       \      /\   \       \     o---->   | ^       # / /   :
    !   /         \    /  \   )       )    <----#   | |  ^ :   / v    :
    !   \         /    \  /  /_______/              v |  ! :          :
    !    \_______/      \/                            o  ! V          :
    !                                                                 :
    !                                                      O          :
    !    .-----------.       .   <.      .>  .           ^  \         :
    !   (             )     (      )    (     )           \  \        :
    !    `-----+ ,---'       `>   '      `  <'             \  v       :
    !          |/                                           *         :
    !   ()     '                                       _        __    :
    !        __   ,-.   .--.   .--.--.     .--.      .' '.    ,'  '.  :
    !   (_) (__) (   ) (    ) (  ( )  )   (    )    (     )  (      ) :
    !             `-'   `--'   `--'--'     `--'      `._.'    `.__.'  :
    !                                                                 !
    !      ___        ____         ____           _____               !
    !    ,'   `.    ,'    `.     .'    `.       ,'     `.             !
    !   /       \  /        \   /        \     /         \            !
    !   \       /  \        /  (          )   (           )           !
    !    `.___.'    `.____.'    \        /     \         /            !
    !                            `.____.'       `._____.'             !
    !        ______                                                   !
    !      ,'      `.                                                 !
    !     /          \    .-----. .----.     ".--------------."       !
    !    |            |    \   /   \    \    "| Don't draw me|"       !
    !    |            |     \ /     \    \   "|              |"       !
    !     \          /       '       '----'  "'--------------'"       !
    !      `.______.'                                                 !
    !                                                                 !
    `~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~'


 o->  Quick logo scribbles
        .---.                      _
       /-o-/--       .--.         |-|               .--.
    .-/ / /->       /--. \     .--)-|    .--.-.    //.-.\
   ( *  \/         / O  )|     |  |-|    |->| |   (+(-*-))
    '-.  \        /\ |-//      .  * |    '--'-'    \\'-'/
       \ /        \ '+'/        \__/                '--'
        '          '--'
                                           _____
       .----.               _             /   __)\
       |    |           ,--(_)            |  /  \ \
     __|____|__       _/ .-. \         ___|  |__/ /
    |  ______--|     (_)(   ) )       / (_    _)_/
    `-/.::::.\-'       \ `-'_/       / /  |  |
     '--------'         `--(_)       \ \__/  |
                                      \(_____/

 o->  Even unicode box drawing characters are supported
            ┌─┬┐  ╔═╦╗  ╓─╥╖  ╒═╤╕
            ├─┼┤  ╠═╬╣  ╟─╫╢  ╞═╪╡
            └─┴┘  ╚═╩╝  ╙─╨╜  ╘═╧╛
            ╭─┬╮
            ├─┼┤
            ╰─┴╯

 o-> Circle, quarter arcs, half circles, 3/4 quarter arcs


       ____            __  __             ____
     ,'    `.        ,'      `.         ,'    `.          __   __
    /        \      /          \       /        \       ,'       `.
    \        /                                         /           \
     `.____.'       \          /       \        /      \           /
                     `.__  __.'         `.____.'        `.__   __.'

       ____             __                  __              ____
     ,'    `.         ,'                      `.          ,'    `.
    /        \       /                          \        /        \
    \                \        /        \        /                 /
     `.__             `.____.'          `.____.'              __.'


 o-> Grids



    .----.        .----.
   /      \      /      \            .-----+-----+-----.
  +        +----+        +----.      |     |     |     |          .-----+-----+-----+-----+
   \      /      \      /      \     |     |     |     |         /     /     /     /     /
    +----+        +----+        +    +-----+-----+-----+        +-----+-----+-----+-----+
   /      \      /      \      /     |     |     |     |       /     /     /     /     /
  +        +----+        +----+      |     |     |     |      +-----+-----+-----+-----+
   \      /      \      /      \     +-----+-----+-----+     /     /     /     /     /
    '----+        +----+        +    |     |     |     |    +-----+-----+-----+-----+
          \      /      \      /     |     |     |     |   /     /     /     /     /
           '----'        '----'      '-----+-----+-----'  '-----+-----+-----+-----+





       ___     ___      .---+---+---+---+---.     .---+---+---+---.  .---.   .---.
   ___/   \___/   \     |   |   |   |   |   |    / \ / \ / \ / \ /   |   +---+   |
  /   \___/   \___/     +---+---+---+---+---+   +---+---+---+---+    +---+   +---+
  \___/   \___/   \     |   |   |   |   |   |    \ / \ / \ / \ / \   |   +---+   |
  /   \___/   \___/     +---+---+---+---+---+     +---+---+---+---+  +---+   +---+
  \___/   \___/   \     |   |   |   |   |   |    / \ / \ / \ / \ /   |   +---+   |
      \___/   \___/     '---+---+---+---+---'   '---+---+---+---'    '---'   '---'


 o-> Graphics Diagram

                                                                             *
    0       3                          P *              Eye /         ^     /
     *-------*      +y                    \                +)          \   /  Reflection
  1 /|    2 /|       ^                     \                \           \ v
   *-+-----* |       |                v0    \       v3           --------*--------
   | |4    | |7      | ◄╮               *----\-----*
   | *-----|-*     ⤹ +-----> +x        /      v X   \          ,-.<--------        o
   |/      |/       / ⤴               /        o     \        ( / ) Refraction    / \
   *-------*       v                 /                \        `-'               /   \
  5       6      +z              v1 *------------------* v2    |                o-----o
                                                               v


 o-> CJK characters

           .------------. | .-----------. |  .-----.
           |  文件系统  | | |   调度器  | |  | MMU |
           '------------' | '-----------' |  '-----'

 o->  Sequence Diagrams

                                   .--->  F
          A       B      C  D     /
          *-------*-----*---*----*----->  E
                   \            ^ \
                    v          /   '--->  G
                     B --> C -'


                          ,-.
                          `-'
                          /|\
         ,---.             |
         |Bob|            / \
         `-+-'           Alice
           |    hello      |
           |-------------->|
           |               |
           |  Is it ok?    |
           |<- - - - - - - |
         ,-+-.           Alice
         |Bob|            ,-.
         `---'            `-'
                          /|\
                           |
                          / \


              ,─.
             ( 0 )
              `-'
            /     \
           /       \
          V         V
         ,─.         ,─.
        ( 1 )       ( 4 )
         `-'         `-' .
       /   \         |  \ `.
      /     \        |   \  `.
     V       V       |    \   `.
    ,─.      ,─.     V     V    V
   ( 2 )    ( 3 )    ,─.   ,─.   ,─.
    `─'      `─'    ( 5 ) ( 6 ) ( 7 )
                     `─'   `─'   `─'



 o-> Plot diagrams

        ▲
    Uin ┊   .------------------------
        ┊   |
        ┊   |
        *---'┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▶

     Udc▲
"Udc_OK"┊      .---------------------
        ┊     /  :
        ┊    /   :
        *---'┄┄┄┄:┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄▶
                 :<----->:
        ▲          500ms :
        ┊                :
Cpu.Qon ┊┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄.-----------
        ┊                |  Inactive
        ┊    Active      |
        *----------------'┄┄┄┄┄┄┄┄┄┄┄▶

 o-> Railroad diagrams

               ┌------┐   .-.  ┌---┐
        o--╮---| elem |--( ; )-| n |--╭--o
           |   └------┘   `-'  └---┘  |
           | ╭------>------╮          |
           | |    ┌---┐    |          |
           ╰-╯-╭--| x |--╮-╰----------╯
           |   |  └---┘  |            |
           |   |   .-.   |            |
           |   `--( , )--'            ^
           |       `-'                |
           |  ╭-------->---------╮    |
           |  |   ┌---┐   .-.    |    |
           ╰--╰-╭-| x |--( , )-╮-╯----╯
                | └---┘   `-'  |
                `-------<------'
                                       .------------>---------------.
           ┌-------------┐  .-.   .-.  |  ┌------┐  .-.   ┌-----┐   |    .-.   ┌------┐
      O____| struct_name |_( : )_( | )_◞__| name |_( : )__| tpe |___◟___( | )__| body |______O
        ◝  └-------------┘  `-'   `-'   ◜ └------┘  `-'   └-----┘  ◝     `-'   └------┘  ◜
        |                               |                    .-.   |                     |
        |                               `------------<------( , )--'                     |
        |                                                    `-'                         |
        `--------------------------------------------------------------------------------'

 o-> Statistical charts


    E +-------------------------*--+     E |                         o
    D |-------------------*--*--|--*     D |                   o  o  |  o
    C |-------------*--*  |  |  |  |     C |             o  o  |  |  |  |
    B |-------*--*  |  |  |  |  |  |     B |       o  o  |  |  |  |  |  |
    A +-*--*--+--+--+--+--+--+--+--+     A +-o--o--|--|--|--|--|--|--|--|
        5 10 15 20 25 30 35 40 45 50         5 10 15 20 25 30 35 40 45 50



  85.67 ┤                                       ╭╮
  78.20 ┤                                       ││                  ╭╮
  70.73 ┤                                       ││  ╭╮ ╭╮ ╭╮   ╭╮  ╭╯╰─╮
  63.27 ┤                        ╭╮         ╭─╮ ││ ╭╯╰╮│╰─╯╰╮╭╮│╰──╯   │╭
  55.80 ┤   ╭╮                 ╭╮││╭╮ ╭╮╭╮  │ ╰─╯╰─╯  ││    ││││       ╰╯
  48.33 ┤   │╰╮      ╭──╮      │││││╰╮│╰╯│  │         ╰╯    ╰╯╰╯
  40.87 ┤╭╮ │ ╰╮╭╮  ╭╯  ╰─╮╭╮╭─╯╰╯╰╯ ╰╯  ╰──╯
  33.40 ┤││ │  ╰╯╰╮╭╯     ││╰╯
  25.93 ┤││╭╯     ╰╯      ╰╯
  18.47 ┼╯││
  11.00 ┤ ╰╯
        └───────────┴───────────┴───────────┴───────────┴───────────┴────
      2011        2012        2013        2014        2015        2016







 o->  Flow charts
                      .--.            .---.  .---. .---.  .---.    .---.  .---.
                      |  |   OS API   '---'  '---' '---'  '---'    '---'  '---'
                      v  |              |      |     |      |        |      |
             .-. .-. .-. |              v      v     |      v        |      v
         .-->'-' '-' '-' |            .------------. | .-----------. |  .-----.
         |     \  |  /   |            | Filesystem | | | Scheduler | |  | MMU |
         |      v . v    |            '------------' | '-----------' |  '-----'
         |_______/ \_____|                   |       |      |        |
                 \ /                         v       |      |        v
                  |     ____              .----.     |      |    .---------.
                  '--> /___/              | IO |<----'      |    | Network |
                                          '----'            |    '---------'
                                             |              |         |
                                             v              v         v
                                      .---------------------------------------.
                                      |                  HAL                  |
                                      '---------------------------------------'


 o->  Block diagrams


      vncviewer         .-,(  ),-.
       __  _         .-(          )-.           gateway           vncserver
      [__]|=|  ---->(    internet    )-------> __________ ------> ____   __
     /⠶⠶ /|_|        '-(          ).-'        [_...__...°]       |    | |==|
                         '-.( ).-'                               |____| |  |
                                                                 /⠶⠶⠶ / |__|


                              Valveless --------.
                            Pulsejet engine    /
                                              V
                               _________.------------------+
                    .---------'                           /  -------->
                   /  .-------._________                  \   thrust-->
                  (  (      _________   `-----------o------+  -------->
                   \  `----'         '----'         |
                    `------._  __^___.----.         |
                             ||  |                  |
                   fuel  __^ ||  | ^__spark         |GND
                  intake     ||  |    plug          |
                             ||  |                  |
                             ||  |                  |
                         ____||  `------------.     |
                        / .---'               |     |
                        | |                   |     |              +-+-+-+-+-+
                    .---| |---.   __          |     |              |-+-+-+-+-|
                 ___|  +-+-+--|--o  `---------*-----|--------------O-+-+-+-+-|
       .-------> ___  ||||||| |  power        |     *--------------O-+-+-+-+-|
        \           | ||||||| |  switch       |     |              |-+-+-+-+-|
          Water     `-+-+-+-+-'            +--o-----o--+           +-+-+-+-+-+
          intake       HHO                 |           |
                      Generator            |  +     -  |            Solar panel
                                           +-----------+
                                              Battery

               =======
                =====  symbolic antenna
                 ===
                  =
                  |
                  |          micro henry
                  |          coil w/tuning lug
                  |    .----.
                  |   (.-') |
                  |   (.-') |
                  |   (.-') |      pico farad cap
                  |   (.-'  |    ___  (trimmable)
                  |   |     |   |___|
        PC   ->  .----'-----'---'---'
        Board    `-------------------
                 ground plane (foil)




 o-> Mindmaps

                                            .-->  Alpha
                                           /
                                          .---->  Initial Release
          Planning *-------.             /         \
                            \           /           '---> Patch 1
      Initial research       \         /             \
                *             \       /               '-->  Patch 2
                 \             \     /---------> Beta
                  \             \   /
                   \             o o                      _______
                    \          .---. *--.___             /       \
                     '------> (     )       '------O->  . Release .
                               `---' o                   \_______/
                               o  o o \
                              /    \ \ \
                          .--'      \ \ \
                         /           \ \ '----+->  Push backs
                        /             \ \      \
                       /|              \ \      '----> Setbacks
                      / |               \ \
                     V /|                \ '-----> Reception
                Team  / |                 \
                     v /|                  \
             Worklaod / .                   '-->> Career change
                     V /
                 PTO  /
                     V
                 Bug


 o->  It can do complex stuff such as circuit diagrams


       +10-15V           ___0,047R
      *---------o-----o-|___|-o--o---------o----o-------.
    + |         |     |       |  |         |    |       |
    -===-      _|_    |       | .+.        |    |       |
    -===-      .-.    |       | | | 2k2    |    |       |
    -===-    470| +   |       | | |        |    |      _|_
    - |       uF|     '--.    | '+'       .+.   |      \ / LED
      +---------o        |6   |7 |8    1k | |   |      -+-
             ___|___   .-+----+--+--.     | |   |       |
              -═══-    |            |     '+'   |       |
                -      |            |1     |  |/  BC    |
               GND     |            +------o--+   547   |
                       |            |      |  |`>       |
                       |            |     ,+.   |       |
               .-------+            | 220R| |   o----||-+  IRF9Z34
               |       |            |     | |   |    |+->
               |       |  MC34063   |     `+'   |    ||-+
               |       |            |      |    |       |  BYV29     -12V6
               |       |            |      '----'       o--|<-o----o--X OUT
 6000 micro  - | +     |            |2                  |     |    |
 Farad, 40V ___|_____  |            |--o                C|    |    |
 Capacitor  ~ ~ ~ ~ ~  |            | GND         30uH  C|    |   --- 470
               |       |            |3      1nF         C|    |   ###  uF
               |       |            |-------||--.       |     |    | +
               |       '-----+----+-'           |      GND    |   GND
               |            5|   4|             |             |
               |             |    '-------------o-------------o
               |             |                           ___  |
               `-------------*------/\/\/------------o--|___|-'
                                     2k              |       1k0
                                                    .+.
                                                    | | 5k6 + 3k3
                                                    | | in Serie
                                                    '+'
                                                     |
                                                    GND



o-> Latest addition: Styling of tagged shapes

    .~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~.
    !                                                                     :
    !   +------+   .------.    .------.   .--------------------------.    :
    !   | {r_1}|   | {r_2}|   (        )  |                          |    :
    !   +------+   '------'    '------'   |                          |    :
    !     _______            ________     |                          |    :
    !    /       \      /\   \       \    |                          |    :
    !   /         \    /  \   )       )   |                          |    :
    !   \         /    \  /  /_______/    |                          |    :
    !    \_______/      \/                |                          |    :
    !                                     |     {bigrect}            |    :
    !                                     |                          |    :
    !                                     `--------------------------'    :
    !                                                                     :
    !      ___        ____         ____           ____                    !
    !    ,'   `.    ,'    `.     .'    `.       .'    `.                  !
    !   /   8   \  /   9    \   /        \     /        \                 !
    !   \  {a}  /  \   {b}  /  (    10    )   (    11    )                !
    !    `.___.'    `.____.'    \  {red} /     \  {a,b} /                 !
    !                            `.____.'       `.____.'                  !
    !        ______                                                       !
    !      ,'      `.                                                     !
    !     /          \    .-----. .----.                                  !
    !    |     12     |    \   /   \    \           {container}           !
    !    |    {c}     |     \ /     \    \                                !
    !     \          /       '       '----'                               !
    !      `.______.'                                                     !
    !                                                                     !
    `~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~'



  .-------------.
 / Advantages: /
'-------------'
 -* Plain text format
    Ultimately portable, backward compatible and future proof.
 -* Degrades gracefully
    Even when not using a graphical renderer, it would still looks good
    as text based diagrams. Paste the text in your source code.
 -* Easiest to use. Anyone knows how to edit text.


# Legend:
r_1 = {
    fill: papayawhip;
}
r_2 = {
    fill: crimson;
}
a = {
    stroke-dasharray: 8;
    fill: lightblue;
}
b = {
    stroke: blue;
}
bigrect = {
    fill: yellow;
    stroke: red;
}
red = {
    fill:red;
    stroke:blue;
}
    "#; 

    let diagram_b = r#" 
       A
       |
       |
 <|----+----|>
       |
       |
       _
       V

                                      .-.                           .--------.
                                   .-+   |                         |          |
                               .--+       '--.                     |'--------'|
                              |  Server Cloud |<------------------>| Database |
                               '-------------'                     |          |
                                   ^      ^                         '--------'
                    Internet       |      |                              ^
          .------------------------'      '-------------.                |
          |                                             |                v
          v                                             v              .------.       .------.
     .--------.      WiFi     .--------.  Bluetooth  .-----.          / #  # /|      / #  # /|
     |        |<------------->|        |<---------->|       |        +------+/| LAN +------+/|
     |Windows |               |  OS X  |            |  iOS  |        |      +/|<--->|      +/|
     +--------+               +--------+            |       |        |Ubuntu+/|     |Ubuntu+/|
    /// ____ \\\             /// ____ \\\           |   o   |        |      +/      |      +/
   '------------'           '------------'           '-----'         '------'       '------'
      Laptop 1                 Laptop 2              Tablet 1         Dedicated Server Rack

   ^
   |
   |

       ┏━━━━┳━━━━┳   ┳━━━━┓
       ┃ A₁ ┃ A₂ ┃ ⋯ ┃ Aⱼ ┃ <--- Basis
       ┡━━━━╇━━━━╇   ╇━━━━┩
       │ 16 │  4 │ ⋯ │  9 │
     ⎧ ├────┼────┼   ┼────┤
     │ │  1 │ -2 │ ⋯ │ 10 │
  Xᵢ ⎨ ├────┼────┼   ┼────┤
     │ │  8 │ 52 │ ⋯ │  0 │
     ⎩ ├────┼────┼   ┼────┤
       │ 14 │  0 │ ⋯ │ -1 │
       └────┴────┴   ┴────┘


  ---|>

           .-.           .-.           .-.           .-.           .-.           .-.
          |   |         |   |         |   |         |   |         |   |         |   |
       .---------.   .--+---+--.   .--+---+--.   .--|   |--.   .--+   +--.   .------|--.
      |           | |           | |   |   |   | |   |   |   | |           | |   |   |   |
       '---------'   '--+---+--'   '--+---+--'   '--|   |--'   '--+   +--'   '--|------'
          |   |         |   |         |   |         |   |         |   |         |   |
           '-'           '-'           '-'           '-'           '-'           '-'

                                 ____                      *
                                |    |_____+---.           |
                                o     _____|    )----------)-------.
                               / \   |     +---'           |     __|__
                              /___\  |                     |     \   /
                                |    '-------------.       |      \ /
              A ----------------'                  |       |       o
                   .-------------------.     o-----)-------'       |
                   |                   |___+---.   |               |___+---.
              B ---*---.__+---.         ___|    )--*--.__++---.     ____)   )----- Y
                        __|    )o---*--'   +---'    ______))   )---'   +---'
              C -------'  +---'     |              |     ++---'
                                    |              o
                                    |             / \
                                    |            /___\
                                    |              |
                                    '--------------'

          .               .                .               .--- 1          .-- 1     / 1
         / \              |                |           .---+            .-+         +
        /   \         .---+---.         .--+--.        |   '--- 2      |   '-- 2   / \ 2
       +     +        |       |        |       |    ---+            ---+          +
      / \   / \     .-+-.   .-+-.     .+.     .+.      |   .--- 3      |   .-- 3   \ / 3
     /   \ /   \    |   |   |   |    |   |   |   |     '---+            '-+         +
     1   2 3   4    1   2   3   4    1   2   3   4         '--- 4          '-- 4     \ 4

.-------------------.                           ^                      .---.
|    A Box          |__.--.__    __.-->         |                      |   |
|                   |        '--'               v                      |   |
'-------------------'                                                  |   |
                       Round                                       *---(-. |
  .-----------------.  .-------.    .----------.         .-------.     | | |
 |   Mixed Rounded  | |         |  / Diagonals  \        |   |   |     | | |
 | & Square Corners |  '--. .--'  /              \       |---+---|     '-)-'       .--------.
 '--+------------+-'  .--. |     '-------+--------'      |   |   |       |        / Search /
    |            |   |    | '---.        |               '-------'       |       '-+------'
    |<---------->|   |    |      |       v                Interior                 |     ^
    '           <---'      '----'   .-----------.              ---.     .---       v     |
 .------------------.  Diag line    | .-------. +---.              \   /           .     |
 |   if (a > b)     +---.      .--->| |       | |    | Curved line  \ /           / \    |
 |   obj->fcn()     |    \    /     | '-------' |<--'                +           /   \   |
 '------------------'     '--'      '--+--------'      .--. .--.     |  .-.     +Done?+-'
    .---+-----.                        |   ^           |\ | | /|  .--+ |   |     \   /
    |   |     | Join                   |   | Curved    | \| |/ | |    \    |      \ /
    |   |     +---->  |                 '-'  Vertical  '--' '--'  '--  '--'        +  .---.
    '---+-----'       |                                                            |  | 3 |
                      v                             not:line    'quotes'        .-'   '---'
                  .---+--------.            /            A || B   *bold*       |        ^
                 |   Not a dot  |      <---+---<--    A dash--is not a line    v        |
                  '---------+--'          /           Nor/is this.            ---


  <|---


    _|_
    \ /
     '

  +-------+         +-----------+
  | Base  |         | Interface |
  +---^---+         +-----^-----+
     /_\                 /_\     _
      |                   :     (_) OtherInterface
      |                   :      |
      |                   :      |
 +---------+      +----------------+
 | Derived |      | Implementation |
 +---------+      +----------------+

                         |                        |
                   _____\|                        |/_____
                        /|                        |\
                         |                        |
    -----------                           __________
         ^               |           |         ^
        /|\              |           |        /|\
         |              \|/         \|/        |
         |           ----V----   ____V_____    |

Poor man's rail road

Style #1
       ┌------┐   .-.  ┌---┐
o______| elem |__( ; )_| n |_____o
   ◝   └------┘   `-'  └---┘  ◜
   | ╭------>-----╮           |
   | |    ┌---┐   |           |
   ◟_◞____| x |___◟___________◞
   |   ◜  └---┘  ◝            |
   |   |   .-.   |            |
   |   `--( , )--'            ^
   |       `-'                |
   |   ╭-------->--------╮    |
   |   |  ┌---┐   .-.    |    |
   ◟___◞__| x |__( , )___◟____◞
        | └---┘   `-'  |
        `-------<------'

                                   .------------>---------------.
       ┌-------------┐  .-.   .-.  |  ┌------┐  .-.   ┌-----┐   |    .-.   ┌------┐
  O____| struct_name |_( : )_( | )_◞__| name |_( : )__| tpe |___◟___( | )__| body |______O
    ◝  └-------------┘  `-'   `-'   ◜ └------┘  `-'   └-----┘  ◝     `-'   └------┘  ◜
    |                               |                    .-.   |                     |
    |                               `------------<------( , )--'                     |
    |                                                    `-'                         |
    `--------------------------------------------------------------------------------'

Style #2

       ┌------┐   .-.  ┌---┐
o--╮---| elem |--( ; )-| n |--╭--o
   |   └------┘   `-'  └---┘  |
   | ╭------>------╮          |
   | |    ┌---┐    |          |
   ╰-╯-╭--| x |--╮-╰----------╯
   |   |  └---┘  |            |
   |   |   .-.   |            |
   |   `--( , )--'            ^
   |       `-'                |
   |  ╭-------->---------╮    |
   |  |   ┌---┐   .-.    |    |
   ╰--╰-╭-| x |--( , )-╮-╯----╯
        | └---┘   `-'  |
        `-------<------'

                                   .------------>-----------------.
       ┌-------------┐  .-.   .-.  |   ┌------┐  .-.   ┌-----┐    |    .-.   ┌------┐
  O-╮--| struct_name |-( : )-( | )-╰-╮-| name |-( : )--| tpe |--╮-╯---( | )--| body |--╭---O
    |  └-------------┘  `-'   `-'    | └------┘  `-'   └-----┘  |      `-'   └------┘  |
    |                                |                    .-.   |                      |
    |                                `------------<------( , )--'                      |
    |                                                     `-'                          |
    `----------------------------------------------------------------------------------'


        120      90      60
   135    ^     ^     ^     ^ 45
      ^    \    |    /    .'
       `.   \   |   /   .'
         `.  \  |  /  .'
           `. \ | / .'
             `.\_/.'
 180 <---------(_)-------------> 0
             .'/|\`.
           .' / | \ `.
         .'  /  |  \  `.
       .'   /   |   \   `.
      V    /    |    \    `.
   225    V     V     V     V

        240    270   300    315


            120      90      60
       135   ^      ^     ^     ^ 45
  150     ^    \    |    /    .'        ◥ 30
    ◤      `.   \   |   /   .'      ⠠⠐⠈
     ⠈⠐⠠     `.  \  |  /  .'    ⠠⠐⠈
         ⠈⠐⠠   `. \ | / .'  ⠠⠐⠈
             ⠈⠐⠠ `.\_/.'⠠⠐⠈
180 <--------------(_)------------------> 0
              ⠠⠐⠈.'/|\`.⠈⠐⠠
          ⠠⠐⠈  .' / | \ `.  ⠈⠐⠠
 210  ⠠⠐⠈    .'  /  |  \  `.    ⠈⠐⠠
    ◣      .'   /   |   \   `.      ⠈⠐⠠  330
          V    /    |    \    `.        ◢
       225    V     V     V     V
            240    270   300    315

      ^ ^ ^
       \|/
        .
       /|\
      v V v

      ^ ^ ^
       \|/
      <-+->
       /|\
      v V v

       \|/
       -.-
       /|\

        |   \/
       -+-  /\
        |

        |      |    |      |
        +--  --+    +--  --+   +--  --+
                    |      |   |      |

                     |    |  |     |
             .- -.   .-  -.  '-   -'
             |   |

        .-   -.  .-.
        '-   -'  | |  | |
                      '-'

      \      |    /  |
       .     .   .   .
       |    /    |    \

        \    /
         .  .
        /    \

       .    .    .    .
      /|    |\   |\  /|
                 | \/ |
                 | /\ |
      \|   |/    |/  \|
       '   '     '    '

       \
       /

       /
       \


       /      \
      .--    --.
     /          \

       /   \
    --.     .--
     /       \

                       \         /
       --.--  --.--   --.--   --.--
        /        \


        |   |
        .   .
       /|   |\

        |
        .
       / \

       \|/
        .
       /|\


       \|/
      --.--
       /|\

       \|/
      --+--
       /|\

        |/  \|
        .    .
        |    |


       -.  -.
       /     \

        .-  .-
       /     \


       /   /     \    \
      '-  '-------'   -'


       .-.
      (   )
       '-'


       .------.
      (        )
       '------'

        ________
       /       /
      /       /
     /_______/


        ________
        \       \
         \       \
          \_______\



     +---------+
      \         \
       +---------+

         |/     \|
     ----+       +----
         |`>   <'|
           |   |
         | /   \ |
   ------<'     `>----
         |\     /|
           |   |
           |   |
         |.>   <.|
    -----+       +----
         |\     /|
           |   |
           |   |
         |/     \|
   ------<.     .>----
         | \   / |
           |   |
           |   |


         |/     \|
     ----+       +----
         |`>   <'|
           |   |
         | +   + |
   ------<'     `>----
         |\     /|
           |   |
           |   |
         |.>   <.|
    -----+       +----
         |\     /|
           |   |
           |   |
         |/     \|
   ------<.     .>----
         | +   + |
           |   |
           |   |
    "#;

    let diagram_c = r#"
o-> Latest addition: Styling of tagged shapes

    .~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~.
    !                                                                     :
    !   +------+   .------.    .------.   .--------------------------.    :
    !   | {r_1}|   | {r_2}|   (        )  |                          |    :
    !   +------+   '------'    '------'   |                          |    :
    !     _______            ________     |                          |    :
    !    /       \      /\   \       \    |                          |    :
    !   /         \    /  \   )       )   |                          |    :
    !   \         /    \  /  /_______/    |                          |    :
    !    \_______/      \/                |                          |    :
    !                                     |     {bigrect}            |    :
    !                                     |                          |    :
    !                                     `--------------------------'    :
    !                                                                     :
    !      ___        ____         ____           ____                    !
    !    ,'   `.    ,'    `.     .'    `.       .'    `.                  !
    !   /   8   \  /   9    \   /        \     /        \                 !
    !   \  {a}  /  \   {b}  /  (    10    )   (    11    )                !
    !    `.___.'    `.____.'    \  {red} /     \  {a,b} /                 !
    !                            `.____.'       `.____.'                  !
    !        ______                                                       !
    !      ,'      `.                                                     !
    !     /          \    .-----. .----.                                  !
    !    |     12     |    \   /   \    \           {container}           !
    !    |    {c}     |     \ /     \    \                                !
    !     \          /       '       '----'                               !
    !      `.______.'                                                     !
    !                                                                     !
    `~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~'



  .-------------.
 / Advantages: /
'-------------'
 -* Plain text format
    Ultimately portable, backward compatible and future proof.
 -* Degrades gracefully
    Even when not using a graphical renderer, it would still looks good
    as text based diagrams. Paste the text in your source code.
 -* Easiest to use. Anyone knows how to edit text.


# Legend:
r_1 = {
    fill: papayawhip;
}
r_2 = {
    fill: crimson;
}
a = {
    stroke-dasharray: 8;
    fill: lightblue;
}
b = {
    stroke: blue;
}
bigrect = {
    fill: yellow;
    stroke: red;
}
red = {
    fill:red;
    stroke:blue;
}
    "#;

    let diagram_d = r#"
       .------.
      (        )
       '------'

        ________
       /       /
      /       /
     /_______/
   
    "#;

    let settings = Settings::default();

    let html_a = bob_handler(diagram_a, &settings);
    std::fs::write("output_a.html", html_a).unwrap();

    let html_b = bob_handler(diagram_b, &settings);
    std::fs::write("output_b.html", html_b).unwrap();

    let html_c = bob_handler(diagram_c, &settings);
    std::fs::write("output_c.html", html_c).unwrap();

    let html_d = bob_handler(diagram_d, &settings);
    std::fs::write("output_d.html", html_d).unwrap();
}