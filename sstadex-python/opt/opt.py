import paretoset as pareto


def run_pareto(macromodel, final_df):
    opt_parameters = ["area"]
    for opt in [i.name for i in macromodel.opt_specifications]:
        opt_parameters.append(opt)
    new_df = final_df[opt_parameters].copy()

    opt_goal = ["min"]
    for opt in [i.opt_goal for i in macromodel.opt_specifications]:
        opt_goal.append(opt)
    mask = pareto.paretoset(new_df, opt_goal)

    return final_df[mask], mask
