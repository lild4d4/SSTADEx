class Macromodel:
	def __init__(
    	self,
		name = "",
        netlist = None,
        electrical_parameters = {},
        specifications = {},
        electrical_variables = {},
        macromodel_parameters = {},
		submacromodels = [],
		primitives = [],
		A = [],
		X = [],
		Z = [],
		eq_solutions = {},
		nodes = {},
		req_tfs = [],
		transistors = {},
		tfs_sol = []
    ):
		self.name = name
		self.netlist = netlist
		self.electrical_parameters = electrical_parameters
		self.specifications = specifications
		self.electrical_variables = electrical_variables
		self.macromodel_parameters = macromodel_parameters
		self.submacromodels = submacromodels
		self.primitives = primitives
		self.A, self.X, self.Z = A, X, Z
		self.eq_solutions = eq_solutions
		self.nodes = nodes
		self.req_tfs = req_tfs
		self.transistors = transistors
		self.tfs_sol = tfs_sol

	def hasPrimitive(self):
		if len(self.primitives)==0:
			return False
		else:
			return True
		
	def addSubmacromodels(self, submacromodel):
		self.submacromodels.append(submacromodel)

class Primitive:
	def __init__(self, name="", parameters={}):
		self.name = name
		self.parameters = parameters