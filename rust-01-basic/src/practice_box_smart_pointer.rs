// Problem 1: Fix the code below so that it compiles
// Solution:

enum BinaryTree {
    Leaf,
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
}

pub fn solution_p1() {}

// Problem 2: Fix the code by completing the function signature
// Solution:

struct Wrapper {
    data: String,
}

fn modify_data(mut wrapper: Box<Wrapper>) -> Box<Wrapper> {
    wrapper.data = String::from("Modified");
    wrapper
}

pub fn solution_p2() {
    let original_wrapper = Box::new(Wrapper {
        data: String::from("Original"),
    });
    let modified_wrapper = modify_data(original_wrapper);
}


// Problem 3: Complete the code below
// Solution:

#[derive(Debug)]
enum ListNode<T> {
    Node(T, Box<ListNode<T>>),
    None,
}

pub fn solution_p3() {
    // Create a linked list representing: Node(1, Node(2, Node(3, Node(4, None))))
    let list = ListNode::Node(
        1,
        Box::new(ListNode::Node(
            2,
            Box::new(ListNode::Node(
                3,
                Box::new(ListNode::Node(4, Box::new(ListNode::None))),
            )),
        )),
    );
    println!("{:?}", list);
}


// Problem 4: Fix the code by adding the type annotation
// Solution:

struct AudioSample;
struct ImageFile;

trait Media {}

impl Media for AudioSample {}
impl Media for ImageFile {}

pub fn solution_p4() {
    let audio_1 = AudioSample;
    let audio_2 = Box::new(AudioSample);

    let audio_3 = audio_1;
    let audio_4 = audio_2;

    let image_1 = Box::new(ImageFile);

    let media_collection: Vec<Box<dyn Media>> = vec![Box::new(audio_3), audio_4, image_1];
}
