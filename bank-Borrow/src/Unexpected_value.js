const engine = {
    working:true
};

const mustang = {
    name:'Mustang',
    engine: engine
};

const camero = {
    naem: "Camero",
    engine: engine
};

function CheckEngine(car){
    if(car.name==='Mustang'){
        car.working=false;
    }
}