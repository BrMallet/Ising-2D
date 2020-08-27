mod renderer;

use rand::Rng;
use std::borrow::Borrow;

#[derive(Debug, PartialEq)]

// Criaçao do spin

enum SpinValue {
    Positive,
    Negative,
}

//--------------------

// Flip de Spin

fn negate_spin_value(v: &SpinValue) -> SpinValue {
    if *v == SpinValue::Positive {
        SpinValue::Negative
    } else {
        SpinValue::Positive
    }
}

//--------------------------------

// Criação da rede 2d aleatoria

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

//------------------------------

// Inversão de Spin

fn should_change(delta: f64, beta: f64) -> bool {
    let mut rng = rand::thread_rng();
    let change = rng.gen_range(0 as f64, 1 as f64);
    if change < (-delta * beta).exp() { true } else { false }
}

//--------------------------


// Neighbor spin sum

fn neighbor_interection(s: &SpinValue, s_iplus1_j: &SpinValue, s_iminus1_j: &SpinValue, s_i_jplus1: &SpinValue, s_ijminus1: &SpinValue) -> f64 {

    let mut s_0: f64 = 0.0; let mut s_1: f64 = 0.0; let mut s_2: f64 = 0.0; let mut s_3: f64 = 0.0; let mut s_4: f64 = 0.0;

    if *s == SpinValue::Positive { s_0 = 1.0
    } else { s_0 = -1.0 }

    if *s_iplus1_j == SpinValue::Positive { s_1 = 1.0
    } else { s_1 = -1.0 }

    if *s_iminus1_j == SpinValue::Positive { 
        s_2 = 1.0
    } else { 
        s_2 = -1.0 }

    if *s_i_jplus1 == SpinValue::Positive { 
        s_3 = 1.0
    } else { 
        s_3 = -1.0 }

    if *s_ijminus1 == SpinValue::Positive { 
        s_4 = 1.0
    } else { 
        s_4 = -1.0 }

    let s_n: f64 =  s_0 * (s_1 + s_2 + s_3 + s_4);

    s_n

}

// --------------------------------------

// Termos vizinhos

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

//------------------

//Spin Total novo 

fn s_total_novo (s: &SpinValue,beta: f64,h: f64) -> f64 {


    if *s == SpinValue::Positive {
        let h_1: f64 = -2.0*beta*h;  
        h_1    
    } else {
        let h_1: f64 = 2.0*beta*h;        
        h_1
    }
    
}

//----------------------------

//Interaçao visinha nova

fn visinho_novo (s: &SpinValue,s_visinho: &SpinValue,beta: f64,j: f64) -> f64 {
    if *s == *s_visinho {
        let h_1: f64 = -2.0*beta*j;  
        h_1
    } else {
        let h_1: f64 = 2.0*beta*j;  
        h_1 
    }
}

////-------------------------
fn main() {
    /* Passo 1: Criação de uma janela e um objeto renderizador de linhas */
    let (mut renderer, mut window) = renderer::setup_renderer();

   /* Passo 2: Loop infinito renderizando as linhas */
    renderer::render_in_a_loop(&mut window, &mut renderer);
//
//
//    // Montagem da rede
//
//    let net_size_r = 30; let net_size_c = 30;
//    let mut random_network = random_2d_network(net_size_r, net_size_c);
//
//    //--------------------------------
//
//    //Parametros fisicos
//    
//    let j: f64 = 1.0; let h: f64 = 0.1; let t: f64 = 2.7; 
//    let beta: f64 = 1.0/ t ;
//
//    // -----------------------------
//    
//    //Loop -----------------------
//
//    for _z in 0 .. 100000000   {   
//     
//
//        //Spin aleatorio
//
//        let mut rng = rand::thread_rng();
//
//
//        let i_ind: usize = rng.gen_range(0 as usize, net_size_r);
//        let j_ind: usize = rng.gen_range(0 as usize, net_size_c); 
//        let s_ij = random_network[i_ind][j_ind].borrow();
//
//        let i_ind_plus1 = next_index(i_ind, net_size_r);
//        let i_ind_minus1 = prev_index(i_ind, net_size_r);
//        let j_ind_plus1 = next_index(j_ind, net_size_c);
//        let j_ind_minus1 = prev_index(j_ind, net_size_c);
//
//        let s_iplus1_j = random_network[i_ind_plus1][j_ind].borrow();
//        let s_iminus1_j = random_network[i_ind_minus1][j_ind].borrow();
//        let s_i_jplus1 = random_network[i_ind][j_ind_plus1].borrow();
//        let s_ijminus1 = random_network[i_ind][j_ind_minus1].borrow();
//        
//        //-------------------------
//
//        //Calculo de Delta
//
//        let h_1 = s_total_novo(s_ij, beta, h);
//        let h_2 = visinho_novo(s_ij, s_iplus1_j, beta, j);
//        let h_3 = visinho_novo(s_ij, s_iminus1_j, beta, j);
//        let h_4 = visinho_novo(s_ij, s_i_jplus1, beta, j);
//        let h_5 = visinho_novo(s_ij, s_ijminus1, beta, j);
//
//        let delta: f64 = h_1 + h_2 + h_3 + h_4 + h_5 ;
//
//        //--------------------------
//
//        //Inversão do Spin
//
//        if delta <= 0.0 {
//            random_network[i_ind][j_ind] = negate_spin_value(random_network[i_ind][j_ind].borrow())
//        } else {
//            if should_change(delta, beta) {
//                random_network[i_ind][j_ind] = negate_spin_value(random_network[i_ind][j_ind].borrow())
//            }
//        }
//        //------------------------
//    } 
//
//    //------------------------
//
//    //Valores Medios das Grandezas Termodinamicas
//
//    let mut s_total: f64 = 0.0; let mut alpha: f64 = 0.0; let mut h_sqr: f64 = 0.0; let mut s: f64 = 0.0;
//
//        for l in 0..net_size_r {
//            for k in 0..net_size_c {
//                let i_ind = l;
//                let j_ind = k;
//
//                let s_ij = random_network[i_ind][j_ind].borrow();
//
//                if *s_ij == SpinValue::Positive {
//                    s_total = s_total + 1.0 ;
//                    s = 1.0
//                } else {
//                    s_total = s_total - 1.0 ;
//                    s = -1.0
//                }
//
//                let i_ind_plus1 = next_index(i_ind, net_size_r);
//                let i_ind_minus1 = prev_index(i_ind, net_size_r);
//                let j_ind_plus1 = next_index(j_ind, net_size_c);
//                let j_ind_minus1 = prev_index(j_ind, net_size_c);
//
//                let s_iplus1_j = random_network[i_ind_plus1][j_ind].borrow();
//                let s_iminus1_j = random_network[i_ind_minus1][j_ind].borrow();
//                let s_i_jplus1 = random_network[i_ind][j_ind_plus1].borrow();
//                let s_ijminus1 = random_network[i_ind][j_ind_minus1].borrow();                
//                                    
//                let vizinho = neighbor_interection(s_ij, s_iplus1_j, s_iminus1_j, s_i_jplus1, s_ijminus1);
//
//                alpha = (alpha + vizinho)/2.0 ;
//
//                let sqr : f64 = (-j * vizinho - h * s).powf(2.0);
//
//                h_sqr  = sqr + h_sqr
//
//            }
//        }
//    
//    let n_c: f64 = net_size_c as f64;
//    let n_r: f64 = net_size_r as f64;
//
//        
//    let m: f64 = s_total/(n_c*n_r);
//
//    println!("Magnetização média: {:?}", m);
//
//    let h_medio: f64 = (-j * alpha - h * s_total)/(n_c*n_r);
//
//    println!("Energia média: {:?}", h_medio);
//
//    let c_v: f64 = (h_sqr/(n_c*n_r) - (h_medio).powf(2.0))/(t).powf(2.0);
//
//    println!("Calor especifico: {:?}", c_v);
//
//    let x_m: f64 = (n_c*n_r - (m).powf(2.0))/(t).powf(2.0) ;
//
//    println!("Suscetibilidade magnetica: {:?}", x_m);   
//
//    //-------------------------------------------------
}