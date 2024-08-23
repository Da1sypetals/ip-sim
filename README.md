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

## TODO:
- Hessian for contact IP energy (case 3);
- Friction;
- Implement affine body.