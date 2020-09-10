mod renderer;
use rand::Rng;
use std::borrow::Borrow;
use plotters::prelude::*;
use std::error::Error;

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

fn s_num(s: &SpinValue) -> f64 {


    if *s == SpinValue::Positive {
        let h_1: f64 = -1.0;  
        h_1    
    } else {
        let h_1: f64 = 1.0;        
        h_1
    }
    
}

//----------------------------

//Interaçao visinha nova

fn visinho_num (s: &SpinValue,s_visinho: &SpinValue) -> f64 {
    if *s == *s_visinho {
        let h_1: f64 = -1.0;  
        h_1
    } else {
        let h_1: f64 = 1.0;  
        h_1 
    }
}

//-------------------------

//Gráfico


fn grafico(vetor: Vec<Vec<f64>>) -> Result<(), Box<dyn Error>> {    
    
    let root = BitMapBackend::new("grafico/teste.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("Arial", 40))
        .build_ranged(-10..50, -10..50)
        ?;

    ctx.configure_mesh().draw()?;

    let vetor_grafico = vetor.clone() ;

    const data: [(f64 , f64); 200] = vetor_grafico ;

    ctx.draw_series(
        data
            .into_iter()
            .map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )
    ?;

    Ok(())


}




//Main

fn main() {   
    /* Passo 1: Criação de uma janela e um objeto renderizador de linhas */
    let (mut renderer, mut window) = renderer::setup_renderer();

   /* Passo 2: Loop infinito renderizando as linhas */
    renderer::render_in_a_loop(&mut window, &mut renderer);

    // Montagem da rede

    let net_size_r = 40; let net_size_c = 40;    

    //--------------------------------

    //Parametros fisicos
    
    let j: f64 = 1.0; let h: f64 = 0.1;   
     
    for c in 1 .. 100 {          
        


        let mut random_network = random_2d_network(net_size_r, net_size_c);
        let mut t = c as f64 ;
        t = t/10.0 ;
        let beta: f64 = 1.0/ t ;

        let mut data_t1: Vec<f64> = Vec::new();
        let mut data_t2: Vec<f64> = Vec::new();
        let mut data_t3: Vec<f64> = Vec::new();
        let mut data_t4: Vec<f64> = Vec::new();

        data_t1.push(t) ;
        data_t2.push(t) ;
        data_t3.push(t) ;
        data_t4.push(t) ;
        
        let mut data_m: Vec<Vec<f64>> =  Vec::new();

        data_m.push(data_t1);

        let mut data_h_medio: Vec<Vec<f64>> = Vec::new();

        data_h_medio.push(data_t2);

        let mut data_c_v: Vec<Vec<f64>> = Vec::new();

        data_c_v.push(data_t3);

        let mut data_x_m: Vec<Vec<f64>> = Vec::new();
        
        data_x_m.push(data_t4);

    // -----------------------------
    
    //Loop -----------------------

        for _z in 0 .. 10000000   {   
     

        //Spin aleatorio

        let mut rng = rand::thread_rng();


        let i_ind: usize = rng.gen_range(0 as usize, net_size_r);
        let j_ind: usize = rng.gen_range(0 as usize, net_size_c); 
        let s_ij = random_network[i_ind][j_ind].borrow();

        let i_ind_plus1 = next_index(i_ind, net_size_r);
        let i_ind_minus1 = prev_index(i_ind, net_size_r);
        let j_ind_plus1 = next_index(j_ind, net_size_c);
        let j_ind_minus1 = prev_index(j_ind, net_size_c);

        let s_iplus1_j = random_network[i_ind_plus1][j_ind].borrow();
        let s_iminus1_j = random_network[i_ind_minus1][j_ind].borrow();
        let s_i_jplus1 = random_network[i_ind][j_ind_plus1].borrow();
        let s_ijminus1 = random_network[i_ind][j_ind_minus1].borrow();
        
        //-------------------------

        //Calculo de Delta

        let h_1 = s_num(s_ij)*2.0*beta*h;
        let h_2 = visinho_num(s_ij, s_iplus1_j)*2.0*beta*j;
        let h_3 = visinho_num(s_ij, s_iminus1_j)*2.0*beta*j;
        let h_4 = visinho_num(s_ij, s_i_jplus1)*2.0*beta*j;
        let h_5 = visinho_num(s_ij, s_ijminus1)*2.0*beta*j;

        let delta: f64 = h_1 + h_2 + h_3 + h_4 + h_5 ;

        //--------------------------

        //Inversão do Spin

        if delta <= 0.0 {
            random_network[i_ind][j_ind] = negate_spin_value(random_network[i_ind][j_ind].borrow())
        } else {
            if should_change(delta, beta) {
                random_network[i_ind][j_ind] = negate_spin_value(random_network[i_ind][j_ind].borrow())
            }
        }
        //------------------------
        //println!("Numero de interacoes: {:?}", z);
        } 

    //------------------------

    //Valores Medios das Grandezas Termodinamicas

        let mut s_total: f64 = 0.0; let mut alpha: f64 = 0.0; let mut h_sqr: f64 = 0.0;

        for l in 0..net_size_r {
            for k in 0..net_size_c {
                let i_ind = l;
                let j_ind = k;

                let s_ij = random_network[i_ind][j_ind].borrow();

                
                let s = s_num(s_ij);
                s_total = s_total + s ;

                let i_ind_plus1 = next_index(i_ind, net_size_r);
                let i_ind_minus1 = prev_index(i_ind, net_size_r);
                let j_ind_plus1 = next_index(j_ind, net_size_c);
                let j_ind_minus1 = prev_index(j_ind, net_size_c);

                let s_iplus1_j = random_network[i_ind_plus1][j_ind].borrow();
                let s_iminus1_j = random_network[i_ind_minus1][j_ind].borrow();
                let s_i_jplus1 = random_network[i_ind][j_ind_plus1].borrow();
                let s_ijminus1 = random_network[i_ind][j_ind_minus1].borrow();
                
                let h_2 = visinho_num(s_ij, s_iplus1_j);
                let h_3 = visinho_num(s_ij, s_iminus1_j);
                let h_4 = visinho_num(s_ij, s_i_jplus1);
                let h_5 = visinho_num(s_ij, s_ijminus1);

                let vizinho = h_2 + h_3 + h_4 + h_5 ;

                alpha = (alpha + vizinho)/2.0 ;

                let sqr : f64 = (-j * vizinho - h * s).powf(2.0);

                h_sqr  = sqr + h_sqr

            }
        }
    
        let n_c: f64 = net_size_c as f64;
        let n_r: f64 = net_size_r as f64;


        let m: f64 = s_total/(n_c*n_r);        

        data_m[c].push(m) ;

        // println!("Magnetização média: {:?}", m);

        let h_medio: f64 = (-j * alpha - h * s_total)/(n_c*n_r);

       data_h_medio[c].push(h_medio) ;

        // println!("Energia média: {:?}", h_medio);

        let c_v: f64 = (h_sqr/(n_c*n_r) - (h_medio).powf(2.0))/(t).powf(2.0);
        
        data_m[c].push(c_v) ;

        // println!("Calor especifico: {:?}", c_v);

        let x_m: f64 = (n_c*n_r - (m).powf(2.0))/(t).powf(2.0) ;

        data_x_m[c].push(x_m) ;

        // println!("Suscetibilidade magnetica: {:?}", x_m);  


        println!("({:?} , {:?} , {:?} , {:?} , {:?}),", t, m, h_medio, c_v, x_m ); 
        
        data_x_m ;

        if c == 99 {
            grafico(data_x_m).unwrap()
        }



        
        
    } 

    


      //-------------------------------------------------      

}
