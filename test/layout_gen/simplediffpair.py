import sys
from pathlib import Path

import pya

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.append(str(SCRIPT_DIR))
from cont import *


#### FUNCTIONS ####
def ensure_sg13_dev_library_loaded():
    if pya.Library.library_by_name("SG13_dev") is not None:
        return

    pdk_python_dir = (
        SCRIPT_DIR.parent.parent
        / "IHP-Open-PDK"
        / "ihp-sg13g2"
        / "libs.tech"
        / "klayout"
        / "python"
    )

    sys.path.append(str(pdk_python_dir))
    sys.path.append(str(pdk_python_dir / "pycell4klayout-api" / "source" / "python"))

    import sg13g2_pycell_lib  # noqa: F401

    if pya.Library.library_by_name("SG13_dev") is None:
        raise RuntimeError("No se pudo registrar la libreria KLayout 'SG13_dev'")


def gate_center_x(x0, col_idx, length, difusion_hsp, poly_offset):
    gate_origin_x = x0 + poly_offset + col_idx * (difusion_hsp + length)
    return gate_origin_x + length / 2


def gate_contact_origin(x_center, num_row=1, num_col=2):
    contact_width = num_col * 160 + (num_col - 1) * 180
    contact_height = num_row * 160 + (num_row - 1) * 180
    return x_center - contact_width / 2, contact_height


def fill_contact_array(
    cell, layout, x0, y0, x1, y1, contact_width=160, contact_spacing=180
):
    num_col = int(
        max((x1 - x0 + contact_spacing) // (contact_width + contact_spacing), 1)
    )
    num_row = int(
        max((y1 - y0 + contact_spacing) // (contact_width + contact_spacing), 1)
    )

    for row in range(num_row):
        for col in range(num_col):
            cx = x0 + col * (contact_width + contact_spacing) + contact_width / 2
            cy = y0 + row * (contact_width + contact_spacing) + contact_width / 2
            cell.shapes(layout).insert(via_box(cx, cy))


def fill_via_array(
    cell,
    layout,
    x0,
    y0,
    x1,
    y1,
    via_width=160,
    via_spacing=180,
    metal_base=None,
    path_width=None,
):
    num_col = int(max((x1 - x0 + via_spacing) // (via_width + via_spacing), 1))
    num_row = int(max((y1 - y0 + via_spacing) // (via_width + via_spacing), 1))

    print("num_row:", num_row, "num_col:", num_col)

    cell.shapes(metal_base).insert(
        pya.Box(
            x0 - path_width // 2,
            min(y0, y1) - path_width // 2,
            x1 + path_width // 2,
            max(y0, y1),
        )
    )

    for row in range(num_row):
        for col in range(num_col):
            cx = x0 + col * (via_width + via_spacing)
            cy = y0 + row * (via_width + via_spacing)
            cell.shapes(layout).insert(via_box(cx, cy))


VIA_SIZE = 160


def via_box(x_center, y_center, size=VIA_SIZE):
    half = size / 2
    return pya.Box(x_center - half, y_center - half, x_center + half, y_center + half)


def source_center_x(x0, source_idx, length, difusion_hsp, path_width):
    contact_pitch = length + difusion_hsp
    source_x0 = x0 + 2 * source_idx * contact_pitch
    return source_x0 + path_width / 2


def allocate_track(track_usage, key):
    track_idx = track_usage.get(key, 0)
    track_usage[key] = track_idx + 1
    return track_idx


def combine_boxes(boxes):
    left = min(box.left for box in boxes)
    bottom = min(box.bottom for box in boxes)
    right = max(box.right for box in boxes)
    top = max(box.top for box in boxes)
    return pya.Box(left, bottom, right, top)


def translate_box(box, dx=0, dy=0):
    return pya.Box(box.left + dx, box.bottom + dy, box.right + dx, box.top + dy)


def build_device_array_bbox(device_box, row_offsets):
    translated_boxes = [
        translate_box(device_box, dy=row_offset) for row_offset in row_offsets
    ]
    return combine_boxes(translated_boxes)


def ring_anchor_point(ring_data, side, offset=0):
    outer = ring_data["outer_box"]
    if side == "top":
        return pya.Point(
            (outer.left + outer.right) // 2 + offset,
            outer.top - ring_data["ring_width"] // 2,
        )
    if side == "bottom":
        return pya.Point(
            (outer.left + outer.right) // 2 + offset,
            outer.bottom + ring_data["ring_width"] // 2,
        )
    if side == "left":
        return pya.Point(
            outer.left + ring_data["ring_width"] // 2,
            (outer.bottom + outer.top) // 2 + offset,
        )
    if side == "right":
        return pya.Point(
            outer.right - ring_data["ring_width"] // 2,
            (outer.bottom + outer.top) // 2 + offset,
        )
    raise ValueError(f"Lado de ring invalido: {side}")


def create_ring(
    cell,
    horizontal_layer,
    vertical_layer,
    via1_layer,
    inner_box,
    ring_width,
    clearance,
    name="ring",
    top_offset=0,
    bottom_offset=0,
    left_offset=0,
    vtrack_spacing=None,
    right_offset=0,
    track_usage=None,
    num_rows=None,
):
    top_ring_track = allocate_track(track_usage, ("top", 0))
    bottom_ring_track = allocate_track(track_usage, ("bottom", num_rows - 1))
    left_ring_track = allocate_track(track_usage, ("ring_vertical", "left"))
    right_ring_track = allocate_track(track_usage, ("ring_vertical", "right"))

    print("left_ring_track:", left_ring_track)

    outer_box = pya.Box(
        inner_box.left - clearance - left_ring_track * (ring_width + vtrack_spacing),
        inner_box.bottom
        - clearance
        - bottom_ring_track * (ring_width + vtrack_spacing),
        inner_box.right + clearance + right_ring_track * (ring_width + vtrack_spacing),
        inner_box.top + clearance + top_ring_track * (ring_width + vtrack_spacing),
    )

    print("inner_box.top:", top_offset)

    top_box = pya.Box(
        outer_box.left, outer_box.top - ring_width, outer_box.right, outer_box.top
    )

    bottom_box = pya.Box(
        outer_box.left, outer_box.bottom, outer_box.right, outer_box.bottom + ring_width
    )
    left_box = pya.Box(
        outer_box.left, outer_box.bottom, outer_box.left + ring_width, outer_box.top
    )
    right_box = pya.Box(
        outer_box.right - ring_width, outer_box.bottom, outer_box.right, outer_box.top
    )

    cell.shapes(horizontal_layer).insert(top_box)
    cell.shapes(horizontal_layer).insert(bottom_box)
    cell.shapes(vertical_layer).insert(left_box)
    cell.shapes(vertical_layer).insert(right_box)

    cell.shapes(via1_layer).insert(
        via_box(outer_box.left + ring_width // 2, outer_box.top - ring_width // 2)
    )

    cell.shapes(via1_layer).insert(
        via_box(outer_box.left + ring_width // 2, outer_box.bottom + ring_width // 2)
    )

    cell.shapes(via1_layer).insert(
        via_box(outer_box.right - ring_width // 2, outer_box.top - ring_width // 2)
    )

    cell.shapes(via1_layer).insert(
        via_box(outer_box.right - ring_width // 2, outer_box.bottom + ring_width // 2)
    )

    return {
        "name": name,
        "inner_box": inner_box,
        "outer_box": outer_box,
        "ring_width": ring_width,
        "clearance": clearance,
        "layers": {"horizontal": horizontal_layer, "vertical": vertical_layer},
        "segments": {
            "top": top_box,
            "bottom": bottom_box,
            "left": left_box,
            "right": right_box,
        },
    }


def connect_point_to_ring(
    cell,
    m1_layer,
    m2_layer,
    via1_layer,
    start_point,
    ring_data,
    ring_side,
    path_width,
    offset=0,
    vtrack_spacing=None,
    disp_width=None,
):
    print("start_point:", start_point)

    if vtrack_spacing is None:
        vtrack_spacing = path_width + 220

    ring_point = ring_anchor_point(ring_data, ring_side, offset=0)

    if ring_side in {"top", "bottom"}:
        ring_point = pya.Point(start_point.x, ring_point.y)
    else:
        ring_point = pya.Point(ring_point.x, ring_point.y + offset * vtrack_spacing)

    if ring_point.x != start_point.x:
        horizontal_box = pya.Box(
            min(start_point.x, ring_point.x) - path_width // 2,
            start_point.y - path_width // 2,
            max(start_point.x, ring_point.x) + path_width // 2,
            start_point.y + path_width // 2,
        )
        cell.shapes(m1_layer).insert(horizontal_box)
    if ring_point.y != start_point.y:
        vertical_box = pya.Box(
            ring_point.x - path_width // 2,
            min(start_point.y, ring_point.y - path_width // 2),
            ring_point.x + path_width // 2,
            max(start_point.y, ring_point.y + path_width // 2),
        )
        cell.shapes(m2_layer).insert(vertical_box)

    if ring_side in {"top"}:
        # cell.shapes(via1_layer).insert(
        #     via_box(ring_point.x, start_point.y + path_width // 2)
        # )
        fill_via_array(
            cell,
            via1_layer,
            start_point.x,
            start_point.y + path_width // 2,
            start_point.x,
            start_point.y + disp_width,
            metal_base=m1_layer,
            path_width=path_width,
        )
    else:
        cell.shapes(via1_layer).insert(
            via_box(ring_point.x, start_point.y - path_width // 2)
        )
    cell.shapes(via1_layer).insert(via_box(ring_point.x, ring_point.y))

    return ring_point


def gate_terminal_point(
    row_offsets,
    row_idx,
    col_idx,
    x0,
    length,
    difusion_hsp,
    poly_offset,
    disp_width,
    poly_tail,
    side,
):
    x = gate_center_x(x0, col_idx, length, difusion_hsp, poly_offset)
    if side == "top":
        y = row_offsets[row_idx] + disp_width + poly_tail
    elif side == "bottom":
        y = row_offsets[row_idx] - poly_tail
    else:
        raise ValueError(f"Side invalido para gate_terminal_point: {side}")
    return pya.Point(x, y)


def source_terminal_point(
    row_offsets,
    row_idx,
    source_idx,
    x0,
    length,
    difusion_hsp,
    path_width,
    disp_width,
    side,
):
    x = source_center_x(x0, source_idx, length, difusion_hsp, path_width)
    if side == "top":
        y = row_offsets[row_idx]
    elif side == "bottom":
        y = row_offsets[row_idx] + disp_width
    else:
        raise ValueError(f"Side invalido para source_terminal_point: {side}")
    return pya.Point(x, y)


def drain_center_x(x0, drain_idx, length, difusion_hsp, path_width):
    contact_pitch = length + difusion_hsp
    drain_x0 = x0 + (2 * drain_idx + 1) * contact_pitch
    return drain_x0 + path_width / 2


def drain_terminal_point(
    row_offsets,
    row_idx,
    drain_idx,
    x0,
    length,
    difusion_hsp,
    path_width,
    disp_width,
    side,
):
    x = drain_center_x(x0, drain_idx, length, difusion_hsp, path_width)
    if side == "top":
        y = row_offsets[row_idx]
    elif side == "bottom":
        y = row_offsets[row_idx] + disp_width
    else:
        raise ValueError(f"Side invalido para drain_terminal_point: {side}")
    return pya.Point(x, y)


def terminal_y_inside_diffusion(y, side, extension):
    if extension <= 0:
        return y
    if side == "top":
        return y - extension
    if side == "bottom":
        return y + extension
    raise ValueError(f"Side invalido para extension de difusion: {side}")


def connect_vertical_device_terminals(
    cell,
    m1_layer,
    m2_layer,
    via1_layer,
    start_point,
    end_point,
    path_width,
    column_x=None,
    start_side=None,
    end_side=None,
    extend_to_diffusion=False,
    diffusion_extension=0,
):
    print("start_point_vertical_conn", start_point)
    print("end_point_vertical_conn", end_point)

    if column_x is None:
        column_x = int((start_point.x + end_point.x) / 2)

    start_m2_y = start_point.y
    end_m2_y = end_point.y
    if extend_to_diffusion:
        if start_side is None or end_side is None:
            raise ValueError("extend_to_diffusion=True requiere start_side y end_side")
        start_m2_y = terminal_y_inside_diffusion(
            start_point.y, start_side, diffusion_extension
        )
        end_m2_y = terminal_y_inside_diffusion(
            end_point.y, end_side, diffusion_extension
        )

    if start_point.x != column_x:
        cell.shapes(m1_layer).insert(
            pya.Box(
                min(start_point.x, column_x) - path_width // 2,
                start_point.y - path_width // 2,
                max(start_point.x, column_x) + path_width // 2,
                start_point.y + path_width // 2,
            )
        )

    if end_point.x != column_x:
        cell.shapes(m1_layer).insert(
            pya.Box(
                min(end_point.x, column_x) - path_width // 2,
                end_point.y - path_width // 2,
                max(end_point.x, column_x) + path_width // 2,
                end_point.y + path_width // 2,
            )
        )

    cell.shapes(m2_layer).insert(
        pya.Box(
            column_x - path_width // 2,
            min(start_m2_y, end_m2_y),
            column_x + path_width // 2,
            max(start_m2_y, end_m2_y),
        )
    )

    # cell.shapes(via1_layer).insert(via_box(column_x, start_point.y))
    # cell.shapes(via1_layer).insert(via_box(column_x, end_point.y))

    return {
        "column_x": column_x,
        "start_m2_y": start_m2_y,
        "end_m2_y": end_m2_y,
    }


def normalize_gate_ref(gate_ref):
    if isinstance(gate_ref, int):
        return (0, gate_ref)
    if isinstance(gate_ref, (tuple, list)) and len(gate_ref) == 2:
        return (int(gate_ref[0]), int(gate_ref[1]))
    raise ValueError(f"Gate invalido: {gate_ref}. Usa columna o (fila, columna)")


def normalize_gate_groups(gate_groups):
    normalized = []
    for group_idx, group in enumerate(gate_groups):
        if isinstance(group, dict):
            gates = group.get("gates", [])
            side = group.get("side", "top")
            name = group.get("name", f"group_{group_idx}")
            between_rows = group.get("between_rows")
        else:
            gates = group
            side = "top"
            name = f"group_{group_idx}"
            between_rows = None

        gates = [normalize_gate_ref(gate) for gate in gates]
        gates = sorted(set(gates))
        if not gates:
            raise ValueError(f"El grupo {name} no tiene gates")
        if side not in {"top", "middle", "bottom"}:
            raise ValueError(f"El grupo {name} tiene side invalido: {side}")
        if between_rows is not None:
            between_rows = tuple(int(row) for row in between_rows)
            if len(between_rows) != 2:
                raise ValueError(f"between_rows invalido en {name}: {between_rows}")

        normalized.append(
            {
                "name": name,
                "gates": gates,
                "side": side,
                "between_rows": between_rows,
            }
        )

    return normalized


def resolve_middle_rows(group):
    if group["between_rows"] is not None:
        upper_row, lower_row = sorted(group["between_rows"])
        return upper_row, lower_row

    rows = sorted({row for row, _ in group["gates"]})
    if len(rows) != 2:
        raise ValueError(
            f"El grupo {group['name']} con side='middle' debe indicar gates en dos filas o usar between_rows"
        )
    return rows[0], rows[1]


def bus_track_key(group):
    side = group["side"]
    rows = sorted({row for row, _ in group["gates"]})
    if side == "middle":
        return (side,) + resolve_middle_rows(group)
    if len(rows) != 1:
        raise ValueError(
            f"El grupo {group['name']} con side='{side}' debe estar en una sola fila"
        )
    return (side, rows[0])


def gate_connection(
    cell,
    layout,
    poly_layer,
    m1_layer,
    x0,
    row_offsets,
    disp_width,
    length,
    gates_per_row,
    difusion_hsp,
    path_width,
    poly_offset,
    poly_tail,
    gate_groups,
    htrack_spacing=None,
    middle_gap=250,
    track_usage=None,
):
    gate_groups = normalize_gate_groups(gate_groups)
    if htrack_spacing is None:
        htrack_spacing = path_width + 220

    num_rows = len(row_offsets)
    contact_rows = 1
    contact_cols = 2
    if track_usage is None:
        track_usage = {}

    for group in gate_groups:
        for row_idx, col_idx in group["gates"]:
            if row_idx < 0 or row_idx >= num_rows:
                raise ValueError(
                    f"La fila {row_idx} del grupo {group['name']} esta fuera de rango 0..{num_rows - 1}"
                )
            if col_idx < 0 or col_idx >= gates_per_row:
                raise ValueError(
                    f"La columna {col_idx} del grupo {group['name']} esta fuera de rango 0..{gates_per_row - 1}"
                )

    def bus_y_for_group(group, track_idx):
        if group["side"] == "top":
            row_idx = group["gates"][0][0]
            print(track_idx * htrack_spacing)
            return (
                row_offsets[row_idx]
                + disp_width
                + poly_tail
                + middle_gap
                + track_idx * htrack_spacing
            )

        if group["side"] == "bottom":
            row_idx = group["gates"][0][0]
            return (
                row_offsets[row_idx]
                - poly_tail
                - middle_gap
                - path_width
                - track_idx * htrack_spacing
            )

        upper_row, lower_row = resolve_middle_rows(group)
        middle_y = (
            row_offsets[lower_row]
            + disp_width
            + poly_tail
            + middle_gap
            + track_idx * htrack_spacing
        )
        expected_upper_y = (
            row_offsets[upper_row]
            - poly_tail
            - middle_gap
            - path_width
            - track_idx * htrack_spacing
        )
        if middle_y != expected_upper_y:
            raise ValueError(
                "Las filas no estan espaciadas para un bus middle comun. "
                "Ajusta row_offsets para que fila_superior_y = fila_inferior_y + disp_width + 2*poly_tail + path_width + 2*middle_gap"
            )
        return middle_y

    for group in gate_groups:
        key = bus_track_key(group)
        track_idx = allocate_track(track_usage, key)

        bus_y0 = bus_y_for_group(group, track_idx)
        centers_x = [
            gate_center_x(x0, col_idx, length, difusion_hsp, poly_offset)
            for _, col_idx in group["gates"]
        ]
        left_x = min(centers_x) - length / 2
        right_x = max(centers_x) + length / 2

        print(f"bus_box {left_x} {bus_y0} {right_x} {bus_y0 + path_width}")

        bus_box = pya.Box(left_x, bus_y0, right_x, bus_y0 + path_width)
        cell.shapes(m1_layer).insert(bus_box)

        for row_idx, col_idx in group["gates"]:
            center_x = gate_center_x(x0, col_idx, length, difusion_hsp, poly_offset)
            poly_box = pya.Box(
                center_x - length / 2,
                bus_y0,
                center_x + length / 2,
                bus_y0 + path_width,
            )
            cell.shapes(poly_layer).insert(poly_box)

            if group["side"] == "top":
                gate_edge_y = row_offsets[row_idx] + disp_width + poly_tail
                extension_y0 = gate_edge_y
                extension_y1 = bus_y0 + path_width
            elif group["side"] == "bottom":
                gate_edge_y = row_offsets[row_idx] - poly_tail
                extension_y0 = bus_y0
                extension_y1 = gate_edge_y
            else:
                upper_row, lower_row = resolve_middle_rows(group)
                if row_idx == lower_row:
                    gate_edge_y = row_offsets[row_idx] + disp_width + poly_tail
                    extension_y0 = gate_edge_y
                    extension_y1 = bus_y0 + path_width
                else:
                    gate_edge_y = row_offsets[row_idx] - poly_tail
                    extension_y0 = bus_y0
                    extension_y1 = gate_edge_y

            extension_box = pya.Box(
                center_x - length / 2,
                extension_y0,
                center_x + length / 2,
                extension_y1,
            )
            cell.shapes(poly_layer).insert(extension_box)

            contact_x0, contact_height = gate_contact_origin(
                center_x,
                num_row=contact_rows,
                num_col=contact_cols,
            )
            contact_y0 = bus_y0 + (path_width - contact_height) / 2
            # contact_array(
            #     cell,
            #     layout,
            #     contact_x0,
            #     contact_y0,
            #     contact_rows,
            #     contact_cols,
            # )
            fill_contact_array(
                cell,
                layout.layer(6, 0),
                center_x - length / 2 + 160 / 2,
                contact_y0,
                center_x + length / 2,
                contact_y0,
                contact_width=160,
                contact_spacing=180,
            )

    return track_usage


def normalize_source_ref(source_ref):
    if isinstance(source_ref, int):
        return (0, source_ref)
    if isinstance(source_ref, (tuple, list)) and len(source_ref) == 2:
        return (int(source_ref[0]), int(source_ref[1]))
    raise ValueError(f"Source invalido: {source_ref}. Usa indice o (fila, indice)")


def normalize_source_segment(segment, name, segment_idx):
    row = int(segment["row"])
    side = segment["side"]
    if side not in {"top", "bottom"}:
        raise ValueError(
            f"El segmento {segment_idx} de {name} tiene side invalido: {side}"
        )

    sources = [normalize_source_ref(source) for source in segment.get("sources", [])]
    normalized_sources = []
    for source_row, source_idx in sources:
        if source_row != row:
            raise ValueError(
                f"El source {(source_row, source_idx)} del segmento {segment_idx} de {name} no coincide con row={row}"
            )
        normalized_sources.append(source_idx)

    normalized_sources = sorted(set(normalized_sources))
    if not normalized_sources:
        raise ValueError(f"El segmento {segment_idx} de {name} no tiene sources")

    return {"row": row, "side": side, "sources": normalized_sources}


def normalize_vertical_link(link):
    if isinstance(link, int):
        return {"source": int(link), "segments": None}
    if isinstance(link, dict) and "source" in link:
        segments = link.get("segments")
        if segments is not None:
            segments = [int(segment_idx) for segment_idx in segments]
        return {"source": int(link["source"]), "segments": segments}
    raise ValueError(f"Vertical link invalido: {link}")


def normalize_source_groups(source_groups):
    normalized = []
    for group_idx, group in enumerate(source_groups):
        name = group.get("name", f"source_group_{group_idx}")
        segments = [
            normalize_source_segment(segment, name, segment_idx)
            for segment_idx, segment in enumerate(group.get("segments", []))
        ]
        if not segments:
            raise ValueError(f"El grupo {name} no tiene segmentos")

        vertical_links = [
            normalize_vertical_link(link) for link in group.get("vertical_links", [])
        ]
        normalized.append(
            {"name": name, "segments": segments, "vertical_links": vertical_links}
        )

    return normalized


def source_bus_track_key(segment):
    return (segment["side"], segment["row"])


def drain_connection(
    cell,
    layout,
    m1_layer,
    m2_layer,
    via1_layer,
    x0,
    row_offsets,
    disp_width,
    length,
    gates_per_row,
    difusion_hsp,
    path_width,
    poly_tail,
    drain_groups,
    drain_gap=250,
    htrack_spacing=None,
    vtrack_spacing=None,
    track_usage=None,
):
    drain_groups = normalize_source_groups(drain_groups)
    if htrack_spacing is None:
        htrack_spacing = path_width + 220
    if vtrack_spacing is None:
        vtrack_spacing = path_width + 220

    num_rows = len(row_offsets)
    drain_count = gates_per_row // 2
    if track_usage is None:
        track_usage = {}

    def bus_y_for_segment(segment, track_idx):
        if segment["side"] == "top":
            return (
                row_offsets[segment["row"]]
                + disp_width
                + poly_tail
                + drain_gap
                + track_idx * (path_width + htrack_spacing)
            )
        return (
            row_offsets[segment["row"]]
            - poly_tail
            - drain_gap
            - path_width
            - track_idx * (path_width + htrack_spacing)
        )

    for group in drain_groups:
        for segment in group["segments"]:
            if segment["row"] < 0 or segment["row"] >= num_rows:
                raise ValueError(
                    f"La fila {segment['row']} del grupo {group['name']} esta fuera de rango 0..{num_rows - 1}"
                )
            for drain_idx in segment["sources"]:
                if drain_idx < 0 or drain_idx >= drain_count:
                    raise ValueError(
                        f"El drain {drain_idx} del grupo {group['name']} esta fuera de rango 0..{drain_count - 1}"
                    )

    for group in drain_groups:
        segment_geometries = []
        for segment in group["segments"]:
            key = source_bus_track_key(segment)
            track_idx = allocate_track(track_usage, key)

            bus_y0 = bus_y_for_segment(segment, track_idx)
            bus_y1 = bus_y0 + path_width
            centers_x = [
                drain_center_x(x0, drain_idx, length, difusion_hsp, path_width)
                for drain_idx in segment["sources"]
            ]
            left_x = min(centers_x) - path_width / 2
            right_x = max(centers_x) + path_width / 2
            cell.shapes(m1_layer).insert(pya.Box(left_x, bus_y0, right_x, bus_y1))

            bus_center_y = bus_y0 + path_width / 2
            for drain_idx in segment["sources"]:
                center_x = drain_center_x(
                    x0, drain_idx, length, difusion_hsp, path_width
                )
                if segment["side"] == "top":
                    drain_anchor_y = row_offsets[segment["row"]] + path_width / 2
                    fill_via_array(
                        cell,
                        via1_layer,
                        center_x,
                        drain_anchor_y,
                        center_x,
                        drain_anchor_y + disp_width - path_width / 2,
                        via_width=160,
                        via_spacing=180,
                        metal_base=m1_layer,
                        path_width=path_width,
                    )
                    cell.shapes(via1_layer).insert(via_box(center_x, bus_center_y))
                else:
                    drain_anchor_y = (
                        row_offsets[segment["row"]] + disp_width - path_width / 2
                    )
                    fill_via_array(
                        cell,
                        via1_layer,
                        center_x,
                        drain_anchor_y - disp_width + path_width,
                        center_x,
                        drain_anchor_y + path_width / 2,
                        via_width=160,
                        via_spacing=180,
                        metal_base=m1_layer,
                        path_width=path_width,
                    )
                    cell.shapes(via1_layer).insert(via_box(center_x, bus_center_y))

                trunk_y0 = min(drain_anchor_y, bus_center_y)
                trunk_y1 = max(drain_anchor_y, bus_center_y)
                cell.shapes(m2_layer).insert(
                    pya.Box(
                        center_x - path_width / 2,
                        trunk_y0 - path_width / 2,
                        center_x + path_width / 2,
                        trunk_y1 + path_width / 2,
                    )
                )

            segment_geometries.append(
                {
                    "row": segment["row"],
                    "side": segment["side"],
                    "sources": segment["sources"],
                    "bus_center_y": bus_center_y,
                    "left_x": left_x,
                    "right_x": right_x,
                }
            )

        for link in group["vertical_links"]:
            drain_idx = link["source"]
            if drain_idx < 0 or drain_idx >= drain_count:
                raise ValueError(
                    f"El vertical link drain={drain_idx} del grupo {group['name']} esta fuera de rango 0..{drain_count - 1}"
                )

            if link["segments"] is None:
                linked_segments = [
                    segment
                    for segment in segment_geometries
                    if drain_idx in segment["sources"]
                ]
            else:
                linked_segments = [segment_geometries[idx] for idx in link["segments"]]

            if len(linked_segments) < 2:
                raise ValueError(
                    f"El vertical link drain={drain_idx} del grupo {group['name']} necesita al menos dos segmentos"
                )

            center_x = drain_center_x(x0, drain_idx, length, difusion_hsp, path_width)
            center_x = center_x + min(link.get("segments") or [0]) * vtrack_spacing
            y_centers = [segment["bus_center_y"] for segment in linked_segments]
            cell.shapes(m2_layer).insert(
                pya.Box(
                    center_x - path_width / 2,
                    min(y_centers),
                    center_x + path_width / 2,
                    max(y_centers),
                )
            )
            for bus_center_y in y_centers:
                cell.shapes(via1_layer).insert(via_box(center_x, bus_center_y))

    return track_usage


def source_connection(
    cell,
    layout,
    m1_layer,
    m2_layer,
    via1_layer,
    x0,
    row_offsets,
    disp_width,
    length,
    gates_per_row,
    difusion_hsp,
    path_width,
    poly_tail,
    source_groups,
    source_gap=250,
    htrack_spacing=None,
    vtrack_spacing=None,
    track_usage=None,
):
    source_groups = normalize_source_groups(source_groups)
    if htrack_spacing is None:
        htrack_spacing = path_width + 220
    if vtrack_spacing is None:
        vtrack_spacing = path_width + 220

    num_rows = len(row_offsets)
    source_count = gates_per_row // 2 + 1
    if track_usage is None:
        track_usage = {}

    def bus_y_for_segment(segment, track_idx):
        if segment["side"] == "top":
            print(
                f"row_offsets[{segment['row']}]={row_offsets[segment['row']]}, disp_width={disp_width}, poly_tail={poly_tail}, source_gap={source_gap}, track_idx={track_idx}, htrack_spacing={htrack_spacing}, path_width={path_width}"
            )

            return (
                row_offsets[segment["row"]]
                + disp_width
                + poly_tail
                + source_gap
                + track_idx * (path_width + htrack_spacing)
            )
        return (
            row_offsets[segment["row"]]
            - poly_tail
            - source_gap
            - path_width
            - track_idx * (path_width + htrack_spacing)
        )

    for group in source_groups:
        for segment in group["segments"]:
            if segment["row"] < 0 or segment["row"] >= num_rows:
                raise ValueError(
                    f"La fila {segment['row']} del grupo {group['name']} esta fuera de rango 0..{num_rows - 1}"
                )
            for source_idx in segment["sources"]:
                if source_idx < 0 or source_idx >= source_count:
                    raise ValueError(
                        f"El source {source_idx} del grupo {group['name']} esta fuera de rango 0..{source_count - 1}"
                    )

    for group in source_groups:
        segment_geometries = []
        for segment in group["segments"]:
            key = source_bus_track_key(segment)
            track_idx = allocate_track(track_usage, key)

            bus_y0 = bus_y_for_segment(segment, track_idx)
            print("bus_y0:", bus_y0)
            bus_y1 = bus_y0 + path_width
            print("bus_y1:", bus_y1)
            centers_x = [
                source_center_x(x0, source_idx, length, difusion_hsp, path_width)
                for source_idx in segment["sources"]
            ]
            left_x = min(centers_x) - path_width / 2
            right_x = max(centers_x) + path_width / 2
            cell.shapes(m1_layer).insert(pya.Box(left_x, bus_y0, right_x, bus_y1))

            bus_center_y = bus_y0 + path_width / 2
            for source_idx in segment["sources"]:
                center_x = source_center_x(
                    x0, source_idx, length, difusion_hsp, path_width
                )
                if segment["side"] == "top":
                    source_anchor_y = row_offsets[segment["row"]] + path_width / 2
                    fill_via_array(
                        cell,
                        via1_layer,
                        center_x,
                        source_anchor_y,
                        center_x,
                        source_anchor_y + disp_width - path_width / 2,
                        via_width=160,
                        via_spacing=180,
                        metal_base=m1_layer,
                        path_width=path_width,
                    )
                    cell.shapes(via1_layer).insert(via_box(center_x, bus_center_y))
                else:
                    source_anchor_y = (
                        row_offsets[segment["row"]] + disp_width - path_width / 2
                    )
                    fill_via_array(
                        cell,
                        via1_layer,
                        center_x,
                        source_anchor_y - disp_width + path_width,
                        center_x,
                        source_anchor_y + path_width / 2,
                        via_width=160,
                        via_spacing=180,
                        metal_base=m1_layer,
                        path_width=path_width,
                    )
                    cell.shapes(via1_layer).insert(via_box(center_x, bus_center_y))

                center_x = center_x

                print(
                    f"Conectando source {source_idx} en fila {segment['row']} al bus en y={bus_center_y} con anchor en y={source_anchor_y}"
                )

                trunk_y0 = min(source_anchor_y, bus_center_y)
                trunk_y1 = max(source_anchor_y, bus_center_y)
                cell.shapes(m2_layer).insert(
                    pya.Box(
                        center_x - path_width / 2,
                        trunk_y0 - path_width / 2,
                        center_x + path_width / 2,
                        trunk_y1 + path_width / 2,
                    )
                )
                # cell.shapes(via1_layer).insert(via_box(center_x, source_anchor_y))

            segment_geometries.append(
                {
                    "row": segment["row"],
                    "side": segment["side"],
                    "sources": segment["sources"],
                    "bus_center_y": bus_center_y,
                    "left_x": left_x,
                    "right_x": right_x,
                }
            )

        for link in group["vertical_links"]:
            source_idx = link["source"]
            if source_idx < 0 or source_idx >= source_count:
                raise ValueError(
                    f"El vertical link source={source_idx} del grupo {group['name']} esta fuera de rango 0..{source_count - 1}"
                )

            if link["segments"] is None:
                linked_segments = [
                    segment
                    for segment in segment_geometries
                    if source_idx in segment["sources"]
                ]
            else:
                linked_segments = [segment_geometries[idx] for idx in link["segments"]]

            if len(linked_segments) < 2:
                raise ValueError(
                    f"El vertical link source={source_idx} del grupo {group['name']} necesita al menos dos segmentos"
                )

            center_x = source_center_x(x0, source_idx, length, difusion_hsp, path_width)
            center_x = center_x + min(link.get("segments") or [0]) * vtrack_spacing
            y_centers = [segment["bus_center_y"] for segment in linked_segments]
            cell.shapes(m2_layer).insert(
                pya.Box(
                    center_x - path_width / 2,
                    min(y_centers),
                    center_x + path_width / 2,
                    max(y_centers),
                )
            )
            for bus_center_y in y_centers:
                cell.shapes(via1_layer).insert(via_box(center_x, bus_center_y))

    return track_usage


def main():
    #### PARAMETERS ######
    x0 = 0

    disp_width = 3750  # nanometers
    disp_length = 2000
    gates_per_row = 4
    num_rows = 2
    poly_tail = 180
    poly_offset = 340

    difusion_hsp = 380
    path_width = 300
    middle_gap = 220
    ring_clearance = 0
    htrack_spacing = 220
    vtrack_spacing = 220

    row_pitch = disp_width + 2 * poly_tail + path_width + 2 * middle_gap
    print("row_pitch:", row_pitch)
    row_offsets = [row_pitch, 0]

    gate_groups = [
        {"name": "VG_TOP", "gates": [(0, 0), (0, 3)], "side": "top"},
        {
            "name": "VG_MID",
            "gates": [(1, 0), (0, 1), (0, 2), (1, 3)],
            "side": "middle",
            "between_rows": (0, 1),
        },
        {"name": "VG_BOT", "gates": [(1, 1), (1, 2)], "side": "bottom"},
    ]

    source_groups = [
        {
            "name": "VS_OUTER",
            "segments": [
                {"row": 0, "sources": [(0, 0), (0, 1)], "side": "top"},
                {"row": 1, "sources": [(1, 0), (1, 2)], "side": "bottom"},
            ],
        },
        {
            "name": "VS_INNER",
            "segments": [
                {"row": 0, "sources": [(0, 1), (0, 2)], "side": "top"},
            ],
        },
    ]

    drain_groups = [
        {
            "name": "VS_OUTER",
            "segments": [
                {"row": 1, "sources": [(1, 0), (1, 1)], "side": "bottom"},
            ],
        },
    ]

    #### GENERATION ####

    ensure_sg13_dev_library_loaded()
    ly = pya.Layout()
    poly_layer = ly.layer(5, 0)
    m1_layer = ly.layer(8, 0)
    via1_layer = ly.layer(19, 0)
    m2_layer = ly.layer(10, 0)

    disp = ly.create_cell(
        "nmos",
        "SG13_dev",
        {
            "l": disp_length * 1e-9,
            "w": gates_per_row * disp_width * 1e-9,
            "ng": gates_per_row,
        },
    )
    if disp is None:
        raise RuntimeError("No se pudo crear el PCell 'nmos' de la libreria 'SG13_dev'")

    top = ly.create_cell("TOP")
    for row_idx in range(num_rows):
        top.insert(
            pya.CellInstArray(disp.cell_index(), pya.Trans(x0, row_offsets[row_idx]))
        )

    device_bbox = build_device_array_bbox(disp.bbox(), row_offsets)

    track_usage = {}

    track_usage = gate_connection(
        top,
        ly,
        poly_layer,
        m1_layer,
        x0,
        row_offsets,
        disp_width,
        disp_length,
        gates_per_row,
        difusion_hsp,
        path_width,
        poly_offset,
        poly_tail,
        gate_groups=gate_groups,
        htrack_spacing=htrack_spacing,
        middle_gap=middle_gap,
        track_usage=track_usage,
    )

    source_connection(
        top,
        ly,
        m1_layer,
        m2_layer,
        via1_layer,
        x0,
        row_offsets,
        disp_width,
        disp_length,
        gates_per_row,
        difusion_hsp,
        path_width,
        poly_tail,
        source_groups=source_groups,
        source_gap=middle_gap,
        htrack_spacing=htrack_spacing,
        vtrack_spacing=vtrack_spacing,
        track_usage=track_usage,
    )

    connect_vertical_device_terminals(
        top,
        m1_layer,
        m2_layer,
        via1_layer,
        start_point=drain_terminal_point(
            row_offsets,
            row_idx=1,
            drain_idx=0,
            x0=x0,
            length=disp_length,
            difusion_hsp=difusion_hsp,
            path_width=path_width,
            disp_width=disp_width,
            side="bottom",
        ),
        end_point=drain_terminal_point(
            row_offsets,
            row_idx=0,
            drain_idx=0,
            x0=x0,
            length=disp_length,
            difusion_hsp=difusion_hsp,
            path_width=path_width,
            disp_width=disp_width,
            side="top",
        ),
        path_width=path_width,
        extend_to_diffusion=False,
        diffusion_extension=0,
    )

    top_ring_track = allocate_track(track_usage, ("top", 0))
    bottom_ring_track = allocate_track(track_usage, ("bottom", num_rows - 1))
    left_ring_track = allocate_track(track_usage, ("ring_vertical", "left"))
    right_ring_track = allocate_track(track_usage, ("ring_vertical", "right"))

    ring_data = create_ring(
        top,
        m1_layer,
        m2_layer,
        via1_layer,
        device_bbox,
        ring_width=path_width,
        clearance=ring_clearance,
        name="outer_ring",
        top_offset=top_ring_track * htrack_spacing,
        bottom_offset=bottom_ring_track * htrack_spacing,
        left_offset=left_ring_track * vtrack_spacing,
        vtrack_spacing=vtrack_spacing,
        right_offset=right_ring_track * vtrack_spacing,
        track_usage=track_usage,
        num_rows=num_rows,
    )

    connect_point_to_ring(
        top,
        m1_layer,
        m2_layer,
        via1_layer,
        drain_terminal_point(
            row_offsets,
            row_idx=0,
            drain_idx=0,
            x0=x0,
            length=disp_length,
            difusion_hsp=difusion_hsp,
            path_width=path_width,
            disp_width=disp_width,
            side="top",
        ),
        ring_data,
        ring_side="top",
        path_width=path_width,
        offset=-1,
        vtrack_spacing=vtrack_spacing,
        disp_width=disp_width,
    )

    connect_point_to_ring(
        top,
        m1_layer,
        m2_layer,
        via1_layer,
        drain_terminal_point(
            row_offsets,
            row_idx=0,
            drain_idx=1,
            x0=x0,
            length=disp_length,
            difusion_hsp=difusion_hsp,
            path_width=path_width,
            disp_width=disp_width,
            side="top",
        ),
        ring_data,
        ring_side="top",
        path_width=path_width,
        offset=-1,
        vtrack_spacing=vtrack_spacing,
        disp_width=disp_width,
    )

    connect_point_to_ring(
        top,
        m1_layer,
        m2_layer,
        via1_layer,
        source_terminal_point(
            row_offsets,
            row_idx=1,
            source_idx=1,
            x0=x0,
            length=disp_length,
            difusion_hsp=difusion_hsp,
            path_width=path_width,
            disp_width=disp_width,
            side="bottom",
        ),
        ring_data,
        ring_side="bottom",
        path_width=path_width,
        offset=-1,
        vtrack_spacing=vtrack_spacing,
        disp_width=disp_width,
    )

    # connect_point_to_ring(
    #     top,
    #     m1_layer,
    #     m2_layer,
    #     via1_layer,
    #     drain_terminal_point(
    #         row_offsets,
    #         row_idx=0,
    #         drain_idx=1,
    #         x0=x0,
    #         length=disp_length,
    #         difusion_hsp=difusion_hsp,
    #         path_width=path_width,
    #         disp_width=disp_width,
    #         side="top",
    #     ),
    #     ring_data,
    #     ring_side="top",
    #     path_width=path_width,
    #     offset=0,
    #     vtrack_spacing=vtrack_spacing,
    # )

    # connect_point_to_ring(
    #     top,
    #     m1_layer,
    #     m2_layer,
    #     via1_layer,
    #     gate_terminal_point(
    #         row_offsets,
    #         row_idx=1,
    #         col_idx=1,
    #         x0=x0,
    #         length=disp_length,
    #         difusion_hsp=difusion_hsp,
    #         poly_offset=poly_offset,
    #         disp_width=disp_width,
    #         poly_tail=poly_tail,
    #         side="bottom",
    #     ),
    #     ring_data,
    #     ring_side="bottom",
    #     path_width=path_width,
    #     offset=-1,
    #     vtrack_spacing=vtrack_spacing,
    # )

    # top_ring_track = allocate_track(track_usage, ("top", 0))
    # bottom_ring_track = allocate_track(track_usage, ("bottom", num_rows - 1))
    # left_ring_track = allocate_track(track_usage, ("ring_vertical", "left"))
    # right_ring_track = allocate_track(track_usage, ("ring_vertical", "right"))

    # ring_data = create_ring(
    #     top,
    #     m1_layer,
    #     m2_layer,
    #     device_bbox,
    #     ring_width=path_width,
    #     clearance=ring_clearance,
    #     name="outer_ring_2",
    #     top_offset=top_ring_track * htrack_spacing,
    #     bottom_offset=bottom_ring_track * htrack_spacing,
    #     left_offset=left_ring_track * vtrack_spacing,
    #     right_offset=right_ring_track * vtrack_spacing,
    # )

    ring_data = create_ring(
        top,
        m1_layer,
        m2_layer,
        via1_layer,
        device_bbox,
        ring_width=path_width,
        clearance=ring_clearance,
        name="outer_ring_2",
        top_offset=top_ring_track * htrack_spacing,
        bottom_offset=bottom_ring_track * htrack_spacing,
        left_offset=left_ring_track * vtrack_spacing,
        vtrack_spacing=vtrack_spacing,
        right_offset=right_ring_track * vtrack_spacing,
        track_usage=track_usage,
        num_rows=num_rows,
    )

    print("Final connection")

    # connect_point_to_ring(
    #     top,
    #     m1_layer,
    #     m2_layer,
    #     via1_layer,
    #     drain_terminal_point(
    #         row_offsets,
    #         row_idx=1,
    #         drain_idx=0,
    #         x0=x0,
    #         length=disp_length,
    #         difusion_hsp=difusion_hsp,
    #         path_width=path_width,
    #         disp_width=disp_width,
    #         side="bottom",
    #     ),
    #     ring_data,
    #     ring_side="bottom",
    #     path_width=path_width,
    #     offset=-1,
    #     vtrack_spacing=vtrack_spacing,
    #     disp_width=disp_width,
    # )

    drain_connection(
        top,
        ly,
        m1_layer,
        m2_layer,
        via1_layer,
        x0,
        row_offsets,
        disp_width,
        disp_length,
        gates_per_row,
        difusion_hsp,
        path_width,
        poly_tail,
        drain_groups=drain_groups,
        drain_gap=middle_gap,
        htrack_spacing=htrack_spacing,
        vtrack_spacing=vtrack_spacing,
        track_usage=track_usage,
    )

    output_dir = SCRIPT_DIR / "generated"
    output_dir.mkdir(exist_ok=True)
    output = output_dir / "simplediffpair.gds"
    ly.write(str(output))
    print(f"GDS escrito en: {output}")


if __name__ == "__main__":
    main()
