use rand::Rng;
use std::borrow::Borrow;

#[derive(Debug, PartialEq)]
enum SpinValue {
    Positive,
    Negative,
}

fn negate_spin_value(v: &SpinValue) -> SpinValue {
    if *v == SpinValue::Positive {
        SpinValue::Negative
    } else {
        SpinValue::Positive
    }
}

fn random_2d_network(rows: usize, columns: usize) -> Vec<Vec<SpinValue>> {
    let mut rng = rand::thread_rng();

    let mut spins: Vec<Vec<SpinValue>> = Vec::new();
    // Vec::with_capacity(rows);
    for i in 0..rows {
        spins.push(Vec::new());
        for _ in 0..columns {
            let n1: bool = rng.gen();
            spins[i].push( if n1 { SpinValue::Positive } else { SpinValue::Negative } );
        }
    }
    spins
}



fn should_change(delta: f64, h: f64) -> bool {
    let mut rng = rand::thread_rng();
    let change = rng.gen_range(0 as f64, 1 as f64);
    if change < (delta * h).exp() { true } else { false }
}

// Changes ---------------------------------------------------------------------------------

// Neighbor spin sum

fn neighbor_interection(s: &SpinValue, s_iplus1_j: &SpinValue, s_iminus1_j: &SpinValue, s_i_jplus1: &SpinValue, s_ijminus1: &SpinValue) -> f64 {

    let mut s_n: f64 = 0.0; let mut s_0: f64 = 0.0; let mut s_1: f64 = 0.0; let mut s_2: f64 = 0.0; let mut s_3: f64 = 0.0; let mut s_4: f64 = 0.0;

    if *s == SpinValue::Positive { s_0 = 1.0
    } else { s_0 = -1.0 }

    if *s_iplus1_j == SpinValue::Positive { s_1 = 1.0
    } else { s_1 = -1.0 }

    if *s_iminus1_j == SpinValue::Positive { s_2 = 1.0
    } else { s_2 = -1.0 }

    if *s_i_jplus1 == SpinValue::Positive { s_3 = 1.0
    } else { s_3 = -1.0 }

    if *s_ijminus1 == SpinValue::Positive { s_4 = 1.0
    } else { s_4 = -1.0 }

    s_n = s_0 + s_1 + s_2 + s_3 + s_4;

    s_n

}

// --------------------------------------

fn next_index(current_index: usize, max_index: usize) -> usize {
    if current_index == max_index - 1 {
        0
    } else {
        current_index + 1
    }
}

fn prev_index(current_index: usize, max_index: usize) -> usize {
    if current_index == 0 {
        max_index - 1
    } else {
        current_index - 1
    }
}

fn main() {
    // Set realistic parameters here
    let net_size_r = 3; let net_size_c = 3;
    let mut random_network = random_2d_network(net_size_r, net_size_c);
    println!("Random network before: {:?}", random_network);
    let j: f64 = 1.0; let beta: f64 = 2.0; let h: f64 = 3.0;
    // -----------------------------

    let mut thermal_balance = false;
    while !thermal_balance {
        let mut rng = rand::thread_rng();


        let i_ind: usize = rng.gen_range(0 as usize, net_size_r);
        let j_ind: usize = rng.gen_range(0 as usize, net_size_c);


        println!("Indice I: {:?}", i_ind);
        println!("Indice J: {:?}", j_ind);

        let mut e_0: f64 = 0.0; let mut e_1: f64 = 0.0; let mut delta: f64 = 0.0;

        for m in 0..2 {
            let s_ij = random_network[i_ind][j_ind].borrow();


            //Changes -----------------------------------------------------------


            // Neighbors parameter ---------------------

            let i_ind_plus1 = next_index(i_ind, net_size_r);
            let i_ind_minus1 = prev_index(i_ind, net_size_r);
            let j_ind_plus1 = next_index(j_ind, net_size_c);
            let j_ind_minus1 = prev_index(j_ind, net_size_c);

            println!("Indice I+1: {:?}", i_ind_plus1);
            println!("Indice J+1: {:?}", j_ind_plus1);
            println!("Indice I-1: {:?}", i_ind_minus1);
            println!("Indice J-1: {:?}", j_ind_minus1);


            let s_iplus1_j = random_network[i_ind_plus1][j_ind].borrow();
            let s_iminus1_j = random_network[i_ind_minus1][j_ind].borrow();
            let s_i_jplus1 = random_network[i_ind][j_ind_plus1].borrow();
            let s_ijminus1 = random_network[i_ind][j_ind_minus1].borrow();

            println!("Spin I+1 J: {:?}", s_iplus1_j);
            println!("Spin I J+1: {:?}", s_iminus1_j);
            println!("Spin I-1 J: {:?}", s_i_jplus1);
            println!("Spin I J-1: {:?}", s_ijminus1);
            println!("Spin I J: {:?}", s_ij);


            // Total spin sum ---------------

            let mut s_total: f64 = 0.0;

            for l in 0..net_size_r {
                for k in 0..net_size_c {
                    let i_ind = l;
                    let j_ind = k;
                    let s_ij = random_network[i_ind][j_ind].borrow();
                    if *s_ij == SpinValue::Positive {
                        s_total = s_total + 1.0
                    } else {
                        s_total = s_total - 1.0
                    }
                }
            }

            println!("Indice I: {:?}", i_ind);
            println!("Indice J: {:?}", j_ind);

            // Total Energy Sum -----------------

            let alpha = neighbor_interection(s_ij, s_iplus1_j, s_iminus1_j, s_i_jplus1, s_ijminus1);

            let gama = s_total;

            e_0 = beta * (-j * alpha - h * gama);

            println!("E_0:{:?}", e_0);
            println!("Spin total:{:?}", gama);
            println!("Primeiros vizinhos:{:?}", alpha);

            delta = e_1.exp() - e_0.exp();

            e_1 = e_0;

            random_network[i_ind][j_ind] = negate_spin_value(random_network[i_ind][j_ind].borrow());

        }

        println! ("Delta E:{:?}", delta);


        // We should do the calculus again with negate s_ij to get delta



        // ----------------------------------------

        if delta <= 0.0 {
            random_network[i_ind][j_ind] = negate_spin_value(random_network[i_ind][j_ind].borrow())
        } else {
            if should_change(delta, h) {
                random_network[i_ind][j_ind] = negate_spin_value(random_network[i_ind][j_ind].borrow())
            }
        }

        // Then I guess we should decide what to do with thermal_balance
        // But for now this is enough
        thermal_balance = true;
    }
    println!("Random network after: {:?}", random_network);
}
