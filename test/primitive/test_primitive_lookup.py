from sstadex import Macromodel, topdown_prim_lookup, bfs

macro_1 = Macromodel(primitives=["primitive_1", "primitive_2"])
macro_2 = Macromodel(submacromodels=[macro_1])
macro_3 = Macromodel(primitives=["primitive_3"])
macro_4 = Macromodel(submacromodels=[macro_2, macro_3])

#print(topdown_prim_lookup(macro_4))

print(bfs(macro_4))