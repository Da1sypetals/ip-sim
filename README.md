# IP-sim 

IP(Incremental-potential)-based simulation written in rust.

## Note:
- Currently only **body-boundary** collision is implemented.
- Contact-IP entrance: `damped_newton_with_contact.rs` function `fill_frame`
- There are artifacts when two points collide (compared to point-edge collision).

## TODO:
- Hessian for contact IP energy (case 3);
- Friction.
- Body-body collision.