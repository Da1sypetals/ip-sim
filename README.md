# IP-sim 

IP(Incremental-potential)-based simulation written in rust.

## Demos
1. Springs 

![springs](demo/springs.gif)

2. Affine body

![affine](demo/affine.gif)

# Problems:
grad & hess may be wrong (so convergence is not guaranteed): rotation of affine body is strange


## Note:
- Use `vis` visualizer:
    - `./vis <frame_json_path> <pause_between_frames?>`
    - Example: `./vis simsave_20240826_15_44_55/frame_409 true`
- Currently implemented collision types:
    - **body-boundary**
    - **inter-springbody** 
- To implement a new type of contact:
    - Contact-IP entrance: `damped_newton_with_contact.rs` function `fill_frame`;
    - Collision detection: refer to `inter_body.rs`.
- There are artifacts when two points collide (compared to point-edge collision).

## TODO:
1. Test: edge on affine body: contact IP grad&hess
2. Contact: affinebody inter-body, affinebody with springsbody
    - inter-body ccd 
    - inter-body contact pair
3. Implement inter-body (contact and ccd) pair collection;

## Questions
1. How to implement singleton in rust?