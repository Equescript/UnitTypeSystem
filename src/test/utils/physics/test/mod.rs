type Vec3 = nalgebra_glm::DVec3;
type Mat3 = nalgebra_glm::DMat3;

type MaterialPointIndex = usize;

pub struct MaterialPoint {
    mass: f64,
    location: Vec3, // world
    velocity: Vec3,
    force: Vec3,
}



/* pub struct MaterialPointConst {
    mass: f64,
}

pub struct MaterialPointDynamics {
    mass: f64,
    force: Vec<Vec3>, // capacity
}

pub struct MaterialPointKinematics {
    location: Vec3, // world
    velocity: Vec3,
}

pub struct MaterialPoint {
    location: Vec3, // world
    velocity: Vec3,
}

pub struct RigidBody {
    mass_center: MaterialPoint,
    basis: Mat3,
    points: Vec<(MaterialPointIndex, Vec3)>,
}

pub struct SoftBody {
    points: Vec<MaterialPointIndex>,
}

pub struct Spring {
    points: (MaterialPointIndex, MaterialPointIndex),
    stiffness: f64,
    init_length: f64,
}

/* pub struct Spring {
    length: f64,
    force: f64,
    force_direction: Vec3, // points.0 -> points.1  normalized
} */

/* impl Spring {
    pub fn calculate_force(&mut self, const_data: &SpringConst) {
        // self.length =
        self.force = (self.length - const_data.init_length) * const_data.stiffness;
    }
} */

pub struct System {
    points: Vec<MaterialPoint>,
    springs: Vec<Spring>,
}



pub trait DynamicUnit {
    fn refresh(&mut self);
    fn location(&self) -> Vec3;
    fn velocity(&self) -> Vec3;
}

pub enum CalculationUnit {
    SoftBodyPoint,
    RigidBody,
}

 */