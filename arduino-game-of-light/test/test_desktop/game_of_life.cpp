#include<unity.h>
#include<gol.h>
#include<stdio.h>

void test_assert_running_desktop(){
    TEST_ASSERT_TRUE(true);
}

void test_create_gol_grid_of_multiple_size(){
    for (int row = 1; row <= 10; ++row)
    {
        for (int col = 1; col <= 10; ++col)
        {
            gameOfLifeGrid all_dead_grid = gameOfLifeGrid(col,row);
            // Test getter through direct index
            for (int index = 0; index < row*col; ++index){
                TEST_ASSERT_FALSE(all_dead_grid._matrix[index]);
            }
        }
    }
}
void test_give_live(){
    for (int row = 1; row <= 10; ++row)
    {
        for (int col = 1; col <= 10; ++col)
        {
            gameOfLifeGrid all_dead_grid = gameOfLifeGrid(col,row);
            for (int index = 0; index < row*col; ++index){
                all_dead_grid.set_cell_at_index(index, true);
                TEST_ASSERT_TRUE(all_dead_grid._matrix[index]);
            }
        }
    }
}

void test_cell_should_die_of_loneliness(){
    // 0 0 0     0 0 0
    // 0 1 0 ==> 0 0 0
    // 0 0 0     0 0 0
    gameOfLifeGrid gol_grid = gameOfLifeGrid(3, 3);
    gol_grid.set_cell_at_index(4, true);
    TEST_ASSERT_TRUE(gol_grid.get_cell_at_index(4));
    gol_grid.tick();
    for (int index = 0; index <= 8; ++index)
    {
         TEST_ASSERT_FALSE(gol_grid.get_cell_at_index(index));
    }
    // 1 0 0     0 0 0
    // 0 0 0 ==> 0 0 0
    // 0 0 1     0 0 0
    // gameOfLifeGrid matrix2 = gameOfLifeGrid(3, 3);
    // matrix2.set_cell_at_index(0, true);
    // matrix2.set_cell_at_index(8, true);
    // TEST_ASSERT_TRUE(matrix2.get_cell_at_index(0));
    // TEST_ASSERT_TRUE(matrix2.get_cell_at_index(8));
    // matrix2.tick();
    // for (int index = 0; index <= 8; ++index)
    // {
    //      TEST_ASSERT_FALSE(matrix2.get_cell_at_index(index));
    // }
}

// void test_cell_should_die_of_overpopulation(){
//     // 1 1 1     1 0 1
//     // 1 1 1 ==> 1 0 1
//     gameOfLifeGrid matrix = gameOfLifeGrid(2, 3);

//     for (int index = 0; index <= 5; ++index)
//     {
//         matrix.set_cell_at_index(index, true);
//         TEST_ASSERT_TRUE(matrix.get_cell_at_index(index));
//     }

//     matrix.tick();
//     // printf(matrix.to_string());
//     TEST_ASSERT_FALSE(matrix.get_cell_at_index(1));
//     TEST_ASSERT_FALSE(matrix.get_cell_at_index(4));
//     TEST_ASSERT_TRUE(matrix.get_cell_at_index(0));
//     TEST_ASSERT_TRUE(matrix.get_cell_at_index(2));
//     TEST_ASSERT_TRUE(matrix.get_cell_at_index(3));
//     TEST_ASSERT_TRUE(matrix.get_cell_at_index(5));

// }


int main(int argc, char const *argv[])
{
    UNITY_BEGIN();
    RUN_TEST(test_assert_running_desktop);
    RUN_TEST(test_create_gol_grid_of_multiple_size);
    RUN_TEST(test_give_live);
    RUN_TEST(test_cell_should_die_of_loneliness);
    // RUN_TEST(test_cell_should_die_of_overpopulation);
    UNITY_END();
    return 0;
}
