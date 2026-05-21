use executor::{Executor, Pose};

mod move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        // when
        executor.execute("M");
        // then
        let expected_pose = Pose::new(1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn shoud_return_y_minus_1_given_command_is_m_and_facing_is_s() {
        // given
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("M");

        // when
        let expected_pose = Pose::new(0, -1, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn shoud_return_x_minus_1_given_command_is_m_and_facing_is_w() {
        // given
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("M");

        // then
        let expected_pose = Pose::new(-1, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn shoud_return_y_plus_1_given_command_is_m_and_facing_is_n() {
        // given
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("M");

        // then
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}

mod turn_left_tests {
    use super::*;

    #[test]
    fn should_return_facing_n_given_command_is_l_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_e_given_command_is_l_and_facing_is_s() {
        // given
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_s_given_command_is_l_and_facing_is_w() {
        // given
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_w_given_command_is_l_and_facing_is_n() {
        // given
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("L");

        // then
        let expected_pose = Pose::new(0, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }
}

mod turn_right_tests {
    use super::*;

    #[test]
    fn should_return_facing_s_given_command_is_r_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("R");

        // then
        let expected_pose = Pose::new(0, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_w_given_command_is_r_and_facing_is_s() {
        // given
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("R");

        // then
        let expected_pose = Pose::new(0, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_n_given_command_is_r_and_facing_is_w() {
        // given
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("R");

        // then
        let expected_pose = Pose::new(0, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_facing_e_given_command_is_r_and_facing_is_n() {
        // given
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        // when
        executor.execute("R");

        // then
        let expected_pose = Pose::new(0, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }
}

mod back_state_tests {
    use super::*;

    #[test]
    fn should_return_x_minus_1_given_status_is_reverse_command_is_m_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("BM");

        // then
        let expected_pose = Pose::new(-1, 0, 'E');
        assert_eq!(expected_pose, executor.query());

    }

    #[test]
    fn should_return_y_plus_1_given_command_is_bbm_and_facing_is_n() {
        // given
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("BBM");

        // then
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());

    }

    #[test]
    fn should_return_s_given_status_is_reverse_command_is_l_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("BL");

        // then
        let expected_pose = Pose::new(0, 0, 'S');
        assert_eq!(expected_pose, executor.query());

    }
    
    #[test]
    fn should_return_n_given_status_is_reverse_command_is_r_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("BR");

        // then
        let expected_pose = Pose::new(0, 0, 'N');
        assert_eq!(expected_pose, executor.query());

    }
}

mod fast_state_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_2_given_status_is_fast_command_is_m_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("FM");

        // then
        let expected_pose = Pose::new(2, 0, 'E');
        assert_eq!(expected_pose, executor.query());

    }

    #[test]
    fn should_return_x_plus_1_and_n_given_status_is_fast_command_is_l_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("FL");

        // then
        let expected_pose = Pose::new(1, 0, 'N');
        assert_eq!(expected_pose, executor.query());

    }

    #[test]
    fn should_return_x_plus_1_and_s_given_status_is_fast_command_is_r_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("FR");

        // then
        let expected_pose = Pose::new(1, 0, 'S');
        assert_eq!(expected_pose, executor.query());

    }

    #[test]
    fn should_return_x_plus_1_given_command_is_ffm_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("FFM");

        // then
        let expected_pose = Pose::new(1, 0, 'E');
        assert_eq!(expected_pose, executor.query());

    }
    
}


mod back_and_fast_state_tests {
    use super::*;

    #[test]
    fn should_return_x_minus_2_given_status_is_reverse_and_fast_command_is_m_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("BFM");

        // then
        let expected_pose = Pose::new(-2, 0, 'E');
        assert_eq!(expected_pose, executor.query());

    }

    #[test]
    fn should_return_x_minus_1_and_s_given_status_is_reverse_and_fast_command_is_l_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("BFL");

        // then
        let expected_pose = Pose::new(-1, 0, 'S');
        assert_eq!(expected_pose, executor.query());

    }

    #[test]
    fn should_return_x_minus_1_and_n_given_status_is_reverse_and_fast_command_is_r_and_facing_is_e() {
        // given
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        
        // when
        executor.execute("BFR");

        // then
        let expected_pose = Pose::new(-1, 0, 'N');
        assert_eq!(expected_pose, executor.query());

    }
    
}
