# IP-sim 

IP(Incremental-potential)-based simulation written in rust.

## Note:
- Currently implemented collision types:
    - **body-boundary**
    - **inter-springbody** 
- To implement a new type of collision:
    - Contact-IP entrance: `damped_newton_with_contact.rs` function `fill_frame`;
    - Collision detection: refer to `inter_body.rs`.
- There are artifacts when two points collide (compared to point-edge collision).

## Recent TODO:
- replace `ContactPair` with `ContactElem` and run the previous program
    - refactor done, but penetration bugs persists.
    - most likely due to :
        - ACCD bug
        - grad computation bug
- grad and hess for the `Affine` variant in `ContactNode` (implement `ContactElem.distance_grad` and `distance_hess`)
- Implement other affine body

## Further TODO:
- Hessian for contact IP energy (case 3);
- Friction;
- Implement affine body.