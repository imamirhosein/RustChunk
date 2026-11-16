package dev.rustchunk.minecraft1201;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonPrimitive;
import net.minecraft.SharedConstants;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.server.Bootstrap;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.Property;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.Map;
import java.util.TreeMap;

/**
 * Exports only the ordered block-state registry facts needed by RustChunk's
 * 1.20.1 backport. This class is compiled and run against the checksum-pinned,
 * Mojang-mapped 1.20.1 client/server jars. It is not packaged in the game.
 */
public final class Vanilla1201BlockFactsExporter {
    private Vanilla1201BlockFactsExporter() {}

    @SuppressWarnings({"rawtypes", "unchecked"})
    private static String propertyName(Property property, Comparable value) {
        return property.getName(value);
    }

    private static JsonElement descriptor(BlockState state, String blockName) {
        JsonObject properties = new JsonObject();
        for (Map.Entry<Property<?>, Comparable<?>> entry : state.getValues().entrySet()) {
            properties.addProperty(
                entry.getKey().getName(),
                propertyName((Property) entry.getKey(), entry.getValue())
            );
        }
        if (properties.size() == 0) {
            return new JsonPrimitive(blockName);
        }
        JsonObject descriptor = new JsonObject();
        descriptor.addProperty("id", blockName);
        descriptor.add("properties", properties);
        return descriptor;
    }

    private static JsonObject exportFacts() {
        JsonArray states = new JsonArray();
        TreeMap<Integer, BlockState> ordered = new TreeMap<>();
        for (BlockState state : Block.BLOCK_STATE_REGISTRY) {
            int id = Block.getId(state);
            if (id < 0 || ordered.put(id, state) != null) {
                throw new IllegalStateException("Invalid or duplicate block-state ID: " + id);
            }
        }

        if (ordered.isEmpty()) {
            throw new IllegalStateException("Minecraft 1.20.1 block-state registry is empty");
        }

        for (int expectedId = 0; expectedId < ordered.size(); expectedId++) {
            BlockState state = ordered.get(expectedId);
            if (state == null) {
                throw new IllegalStateException("Block-state registry has a gap at ID " + expectedId);
            }
            String blockName = BuiltInRegistries.BLOCK.getKey(state.getBlock()).toString();

            JsonArray row = new JsonArray();
            row.add(blockName);
            row.add(state.isAir());
            row.add(!state.getFluidState().isEmpty());
            row.add(state.toString());
            row.add(descriptor(state, blockName));
            states.add(row);
        }

        TreeMap<String, Integer> defaults = new TreeMap<>();
        for (Block block : BuiltInRegistries.BLOCK) {
            String name = BuiltInRegistries.BLOCK.getKey(block).toString();
            int stateId = Block.getId(block.defaultBlockState());
            if (stateId < 0 || defaults.put(name, stateId) != null) {
                throw new IllegalStateException("Invalid or duplicate block registry entry: " + name);
            }
        }

        JsonObject result = new JsonObject();
        result.add("states", states);
        JsonObject defaultStates = new JsonObject();
        for (Map.Entry<String, Integer> entry : defaults.entrySet()) {
            defaultStates.addProperty(entry.getKey(), entry.getValue());
        }
        result.add("defaults", defaultStates);
        return result;
    }

    public static void main(String[] args) throws Exception {
        if (args.length != 1) {
            throw new IllegalArgumentException("Usage: Vanilla1201BlockFactsExporter <output.json>");
        }
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();

        JsonObject facts = exportFacts();
        Path target = Path.of(args[0]).toAbsolutePath();
        Files.createDirectories(target.getParent());
        Path partial = target.resolveSibling(target.getFileName() + ".partial");
        Gson gson = new GsonBuilder().disableHtmlEscaping().create();
        Files.writeString(partial, gson.toJson(facts) + System.lineSeparator(), StandardCharsets.UTF_8);
        Files.move(partial, target, StandardCopyOption.REPLACE_EXISTING);
    }
}
