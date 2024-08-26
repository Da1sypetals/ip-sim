# IP-sim 

IP(Incremental-potential)-based simulation written in rust.

## Note:
- Use `vis` visualizer:
    - `./vis <frame_json_path> <pause_between_frames?>`
    - Example: `./vis simsave_20240826_15_44_55/frame_409 true`
- Currently implemented collision types:
    - **body-boundary**
    - **inter-springbody** 
- To implement a new type of collision:
    - Contact-IP entrance: `damped_newton_with_contact.rs` function `fill_frame`;
    - Collision detection: refer to `inter_body.rs`.
- There are artifacts when two points collide (compared to point-edge collision).

## Recent TODO:
- Implement hessian for contact IP energy (case 3);
- grad and hess for the `Affine` variant in `ContactNode` (implement `ContactElem.distance_grad` and `distance_hess`)
- Implement affine body

## Further TODO:
- Hessian for contact IP energy (case 3);
- Friction;
- Implement affine body.